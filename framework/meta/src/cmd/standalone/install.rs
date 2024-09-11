mod install_wasm_tools;

use crate::cli_args::{InstallArgs, InstallCommand, InstallWasm32Args, InstallWasmOptArgs};

pub fn install(args: &InstallArgs) {
    let command = args
        .command
        .as_ref()
        .expect("Installation command must be specified after `install` keyword.");

    match command {
        InstallCommand::All => {
            install_wasm32(&InstallWasm32Args::default());
            install_wasm_opt(&InstallWasmOptArgs::default());
        },
        InstallCommand::Wasm32(wam32_args) => install_wasm32(wam32_args),
        InstallCommand::WasmOpt(wasm_opt_args) => install_wasm_opt(wasm_opt_args),
    }
}

fn install_wasm32(_wasm32_args: &InstallWasm32Args) {
    install_wasm_tools::install_wasm32_target();
}

fn install_wasm_opt(_wasm_opt_args: &InstallWasmOptArgs) {
    install_wasm_tools::install_wasm_opt();
}
