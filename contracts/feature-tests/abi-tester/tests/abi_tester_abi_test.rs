use klever_sc::abi::{EnumVariantDescription, TypeContents, TypeNames};
use klever_sc::contract_base::ContractAbiProvider;
use std::{fs, fs::File, io::Write};

use klever_sc_meta::abi_json;
use klever_sc_meta::abi_json::KdaAttributeAbiJson;
use klever_sc_meta::kda_attr_file_json::serialize_kda_attribute_json;
use klever_sc_scenario::ScenarioWorld;

#[test]
fn abi_tester_abi_generated_ok() {
    let blockchain = ScenarioWorld::new();

    // generate ABI
    let multi_contract_config = klever_sc_meta::multi_contract_config::<abi_tester::AbiProvider>(
        blockchain.current_dir().as_path(),
    );

    let main_contract = multi_contract_config.find_contract("abi-tester");
    assert!(!main_contract.settings.external_view);
    let view_contract = multi_contract_config.find_contract("abi-tester-ev");
    assert!(view_contract.settings.external_view);
    assert_eq!(
        view_contract.endpoint_names(),
        vec!["external_view", "payable_any_token", "label_a"]
    );

    let main_contract_abi_json = abi_json::abi_to_json_dummy_environment(&main_contract.abi);
    let view_contract_abi_json = abi_json::abi_to_json_dummy_environment(&view_contract.abi);

    // save generated ABI to disk for easier comparison in case something is off
    let mut file = File::create("abi_tester_generated_main.abi.json").unwrap();
    file.write_all(main_contract_abi_json.as_bytes()).unwrap();
    let mut file = File::create("abi_tester_generated_view.abi.json").unwrap();
    file.write_all(view_contract_abi_json.as_bytes()).unwrap();

    // load expected from disk & check!
    assert_eq!(
        main_contract_abi_json,
        fs::read_to_string("./abi_tester_expected_main.abi.json").unwrap()
    );
    assert_eq!(
        view_contract_abi_json,
        fs::read_to_string("./abi_tester_expected_view.abi.json").unwrap()
    );
}

#[test]
fn abi_tester_kda_attr_abi_generated_ok() {
    let original_abi = abi_tester::AbiProvider::abi();
    let kda_attr = original_abi
        .kda_attributes
        .iter()
        .find(|kda_attr| kda_attr.ticker == "OnlyInKda")
        .unwrap();
    let kda_attr_abi = KdaAttributeAbiJson::new(kda_attr);
    let kda_attr_abi_string = serialize_kda_attribute_json(&kda_attr_abi);

    let mut file = File::create("abi_tester_generated_kda_attr.kda-abi.json").unwrap();
    file.write_all(kda_attr_abi_string.as_bytes()).unwrap();

    assert_eq!(
        kda_attr_abi_string,
        fs::read_to_string("./abi_tester_expected_kda_attr.kda-abi.json").unwrap()
    );
}

#[test]
fn check_multi_contract_config() {
    let blockchain = ScenarioWorld::new();

    let multi_contract_config = klever_sc_meta::multi_contract_config::<abi_tester::AbiProvider>(
        blockchain.current_dir().as_path(),
    );

    let ev_contract = multi_contract_config.find_contract("abi-tester-ev");
    assert!(ev_contract.settings.external_view);
    assert_eq!(
        ev_contract.endpoint_names(),
        vec!["external_view", "payable_any_token", "label_a"]
    );
}

#[test]
fn abi_deserialization_check() {
    let main_json = fs::read_to_string("./abi_tester_expected_main.abi.json").unwrap();
    let main_abi = klever_sc_meta::abi_json::deserialize_abi_from_json(&main_json).unwrap();
    let abi_enum_type = main_abi
        .types
        .get("AbiEnum")
        .unwrap()
        .to_type_description(TypeNames {
            abi: "AbiEnum".to_string(),
            rust: "Enum".to_string(),
        });
    if let TypeContents::Enum(variants) = abi_enum_type.contents {
        assert_eq!(variants.len(), 4);
        assert_eq!(
            variants[0],
            EnumVariantDescription {
                docs: vec![],
                name: "Nothing".to_string(),
                discriminant: 0,
                fields: vec![]
            }
        );
    } else {
        panic!("wrong AbiEnum type contents")
    }
}
