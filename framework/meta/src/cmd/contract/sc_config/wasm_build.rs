use std::{ffi::OsStr, fs, process::Command};

use super::ContractVariant;
use crate::ei_check_json::EiCheckJson;
use crate::report_info_json::ReportInfoJson;
use crate::tools::WasmInfo;
use crate::{
    abi_json::ContractAbiJson,
    cli_args::BuildArgs,
    ei::EIVersion,
    kleversc_file_json::{save_kleversc_file_json, KleverscFileJson},
    print_util::*,
    tools,
};

impl ContractVariant {
    pub fn build_contract(&self, build_args: &BuildArgs, output_path: &str) {
        let mut command = self.compose_build_command(build_args);

        print_build_command(self.wasm_output_name(build_args), &command);

        let exit_status = command
            .spawn()
            .expect("failed to spawn contract build process")
            .wait()
            .expect("contract build process was not running");

        assert!(exit_status.success(), "contract build process failed");

        self.finalize_build(build_args, output_path);
    }

    fn compose_build_command(&self, build_args: &BuildArgs) -> Command {
        let mut command = Command::new("cargo");
        command
            .args(["build", "--target=wasm32-unknown-unknown", "--release"])
            .current_dir(self.wasm_crate_path());
        if build_args.locked {
            command.arg("--locked");
        }
        if let Some(target_dir_wasm) = &build_args.target_dir_wasm {
            command.args(["--target-dir", target_dir_wasm]);
        }
        let rustflags = self.compose_rustflags(build_args);
        if !rustflags.is_empty() {
            command.env("RUSTFLAGS", rustflags);
        }
        command
    }

    fn compose_rustflags(&self, build_args: &BuildArgs) -> Rustflags {
        let mut rustflags = Rustflags::default();

        if !build_args.wasm_symbols {
            rustflags.push_flag("-C link-arg=-s");
        }

        rustflags.push_flag(&format!(
            "-C link-arg=-zstack-size={}",
            self.settings.stack_size
        ));

        if build_args.emit_mir {
            rustflags.push_flag("--emit=mir");
        }

        if build_args.emit_llvm_ir {
            rustflags.push_flag("--emit=llvm-ir");
        }
        rustflags
    }

    fn finalize_build(&self, build_args: &BuildArgs, output_path: &str) {
        self.copy_contracts_to_output(build_args, output_path);
        self.run_wasm_opt(build_args, output_path);
        self.run_wasm2wat(build_args, output_path);
        let wasm_info = self.extract_wasm_info(build_args, output_path);
        self.run_twiggy(build_args, output_path);
        self.pack_kleversc_file(build_args, output_path, wasm_info);
    }

    fn copy_contracts_to_output(&self, build_args: &BuildArgs, output_path: &str) {
        let source_wasm_path = self.wasm_compilation_output_path(&build_args.target_dir_wasm);
        let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));
        print_copy_contract(source_wasm_path.as_str(), output_wasm_path.as_str());
        fs::copy(source_wasm_path, output_wasm_path)
            .expect("failed to copy compiled contract to output directory");
    }

    fn pack_kleversc_file(&self, build_args: &BuildArgs, output_path: &str, wasm_info: WasmInfo) {
        let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));
        let compiled_bytes = fs::read(output_wasm_path).expect("failed to open compiled contract");
        let output_kleversc_path = format!(
            "{output_path}/{}",
            self.kleversc_file_output_name(build_args)
        );
        print_pack_kleversc_file(&output_kleversc_path);
        print_contract_size(compiled_bytes.len());
        let mut abi = ContractAbiJson::from(&self.abi);
        let build_info = core::mem::take(&mut abi.build_info).unwrap();
        let ei_check_json = EiCheckJson::new(&self.settings.check_ei, wasm_info.ei_check);
        let report = ReportInfoJson::new(&wasm_info, ei_check_json);
        let kleversc_file_json = KleverscFileJson {
            build_info,
            abi,
            size: compiled_bytes.len(),
            code: hex::encode(compiled_bytes),
            report,
        };
        save_kleversc_file_json(&kleversc_file_json, output_kleversc_path);
    }

    fn run_wasm_opt(&self, build_args: &BuildArgs, output_path: &str) {
        if !build_args.wasm_opt {
            return;
        }

        let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));
        print_call_wasm_opt(&output_wasm_path);
        tools::run_wasm_opt(output_wasm_path.as_str());
    }

    fn run_wasm2wat(&self, build_args: &BuildArgs, output_path: &str) {
        if !build_args.wat {
            return;
        }

        let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));
        let output_wat_path = format!("{output_path}/{}", self.wat_output_name(build_args));
        print_call_wasm2wat(&output_wasm_path, &output_wat_path);
        tools::wasm_to_wat(output_wasm_path.as_str(), output_wat_path.as_str());
    }

    fn extract_wasm_info(&self, build_args: &BuildArgs, output_path: &str) -> WasmInfo {
        let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));

        if !build_args.extract_imports {
            return WasmInfo::extract_wasm_info(
                &output_wasm_path,
                build_args.extract_imports,
                &self.settings.check_ei,
            )
            .expect("error occurred while extracting imports from .wasm ");
        }

        let output_imports_json_path = format!(
            "{}/{}",
            output_path,
            self.imports_json_output_name(build_args)
        );
        print_extract_imports(&output_imports_json_path);

        let wasm_data =
            WasmInfo::extract_wasm_info(&output_wasm_path, true, &self.settings.check_ei)
                .expect("error occurred while extracting imports from .wasm ");

        write_imports_output(
            output_imports_json_path.as_str(),
            wasm_data.imports.as_slice(),
        );
        print_ei_check(&wasm_data, &self.settings.check_ei);

        wasm_data
    }
}

fn write_imports_output(dest_path: &str, import_names: &[String]) {
    let json = serde_json::to_string_pretty(import_names).unwrap();
    fs::write(dest_path, json).expect("failed to write imports json file");
}

fn print_ei_check(wasm_data: &WasmInfo, check_ei: &Option<EIVersion>) {
    if let Some(ei) = check_ei {
        print_check_ei(ei.name());

        if wasm_data.ei_check {
            print_check_ei_ok();
            return;
        }

        for import_name in &wasm_data.imports {
            if !ei.contains_vm_hook(import_name.as_str()) {
                print_invalid_vm_hook(import_name.as_str(), ei.name());
            }
        }
    } else {
        print_ignore_ei_check();
    }
}

impl ContractVariant {
    fn run_twiggy(&self, build_args: &BuildArgs, output_path: &str) {
        if build_args.has_twiggy_call() {
            let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));

            if build_args.twiggy_top {
                let output_twiggy_top_path =
                    format!("{output_path}/{}", self.twiggy_top_name(build_args));
                tools::twiggy::run_twiggy_top(
                    output_wasm_path.as_str(),
                    output_twiggy_top_path.as_str(),
                );
            }
            if build_args.twiggy_paths {
                let output_twiggy_paths_path =
                    format!("{output_path}/{}", self.twiggy_paths_name(build_args));
                tools::twiggy::run_twiggy_paths(
                    output_wasm_path.as_str(),
                    output_twiggy_paths_path.as_str(),
                );
            }
            if build_args.twiggy_monos {
                let output_twiggy_monos_path =
                    format!("{output_path}/{}", self.twiggy_monos_name(build_args));
                tools::twiggy::run_twiggy_monos(
                    output_wasm_path.as_str(),
                    output_twiggy_monos_path.as_str(),
                );
            }
            if build_args.twiggy_dominators {
                let output_twiggy_dominators_path =
                    format!("{output_path}/{}", self.twiggy_dominators_name(build_args));
                tools::twiggy::run_twiggy_dominators(
                    output_wasm_path.as_str(),
                    output_twiggy_dominators_path.as_str(),
                );
            }
        }
    }
}

/// For convenience, for building rustflags.
#[derive(Default)]
struct Rustflags(String);

impl Rustflags {
    fn push_flag(&mut self, s: &str) {
        if !self.0.is_empty() {
            self.0.push(' ');
        }
        self.0.push_str(s);
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<OsStr> for Rustflags {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}
