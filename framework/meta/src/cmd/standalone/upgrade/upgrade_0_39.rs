use std::path::Path;

use super::{
    upgrade_common::{
        re_generate_wasm_crate, rename_files, replace_in_files, version_bump_in_cargo_toml,
    },
    upgrade_print::*,
};
use crate::{
    folder_structure::{DirectoryType, RelevantDirectory},
    CargoTomlContents,
};
use ruplacer::Query;
use toml::{value::Table, Value};

#[rustfmt::skip]
pub const SCENARIO_FILE_PATTERNS: &[(&str, &str)] = &[
    ("mandos_go", "scenario_go"), 
    ("mandos_rs", "scenario_rs"),
];

/// Migrate `0.38.0` to `0.39.0`, including the version bump.
pub fn upgrade_to_39_0(dir: &RelevantDirectory) {
    if dir.dir_type == DirectoryType::Contract {
        v_0_39_prepare_meta(&dir.path);
        v_0_39_prepare_wasm(&dir.path);
    }
    v_0_39_replace_in_files(&dir.path);
    rename_files(dir.path.as_ref(), SCENARIO_FILE_PATTERNS);

    let (from_version, to_version) = dir.upgrade_in_progress.unwrap();
    version_bump_in_cargo_toml(&dir.path, from_version, to_version);
}

/// Post-processing: re-generate the wasm crates.
pub fn postprocessing_after_39_0(dir: &RelevantDirectory) {
    if dir.dir_type != DirectoryType::Contract {
        return;
    }
    print_postprocessing_after_39_1(dir.path.as_path());
    re_generate_wasm_crate(dir);
}

fn v_0_39_prepare_meta(sc_crate_path: &Path) {
    let cargo_toml_path = sc_crate_path.join("meta/Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let mut meta_cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let deps = meta_cargo_toml.dependencies_mut();

    print_cargo_dep_remove(cargo_toml_path.as_path(), "klever-wasm");
    deps.remove("klever-wasm");

    print_cargo_dep_remove(cargo_toml_path.as_path(), "klever-wasm-debug");
    deps.remove("klever-wasm-debug");

    print_cargo_dep_add(cargo_toml_path.as_path(), "klever-sc-meta");
    let mut meta_dep = Table::new();
    meta_dep.insert("version".to_string(), Value::String("0.39.0".to_string()));
    deps.insert("klever-sc-meta".to_string(), Value::Table(meta_dep));

    meta_cargo_toml.save_to_file(&cargo_toml_path);
}

fn v_0_39_prepare_wasm(sc_crate_path: &Path) {
    let cargo_toml_path = sc_crate_path.join("wasm/Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let mut meta_cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let deps = meta_cargo_toml.dependencies_mut();

    print_cargo_dep_remove(cargo_toml_path.as_path(), "klever-wasm-output");
    deps.remove("klever-wasm-output");

    meta_cargo_toml.save_to_file(&cargo_toml_path);
}

fn v_0_39_replace_in_files(sc_crate_path: &Path) {
    replace_in_files(
        sc_crate_path,
        "*Cargo.toml",
        &[
            Query::substring("klever-wasm-debug", "klever-sc-scenario"),
            Query::substring("klever-wasm-modules", "klever-sc-modules"),
            Query::substring("klever-wasm-node", "klever-sc-wasm-adapter"),
            Query::substring("klever-wasm", "klever-sc"),
        ][..],
    );

    replace_in_files(
        sc_crate_path,
        "*rs",
        &[
            Query::substring("klever_codec", "codec"),
            Query::substring(
                "klever_wasm_debug::meta::perform",
                "klever_sc_meta::cli_main",
            ),
            Query::substring(
                "klever_wasm_debug::mandos_go",
                "klever_sc_scenario::run_go",
            ),
            Query::substring(
                "klever_wasm_debug::mandos_rs",
                "klever_sc_scenario::run_rs",
            ),
            Query::substring("klever_wasm_debug", "klever_sc_scenario"),
            Query::substring("klever_wasm_modules", "klever_sc_modules"),
            Query::substring("klever_wasm_node", "klever_sc_wasm_adapter"),
            Query::substring("klever_wasm", "klever_sc"),
            Query::substring("BlockchainMock", "ScenarioWorld"),
            Query::substring("testing_framework", "whitebox"),
            Query::substring("tx_mock", "whitebox"),
            Query::substring("register_contract_builder", "register_contract"),
        ][..],
    );
}
