use klever_sc::contract_base::CallableContract;
use klever_sc_scenario::api::DebugApi;

#[test]
fn test_function_selector() {
    DebugApi::dummy();
    let use_module = use_module::contract_obj::<DebugApi>();

    assert!(!use_module.call("invalid_endpoint"));

    assert!(use_module.call("call_mod_a"));
    assert!(use_module.call("call_mod_b"));
    assert!(use_module.call("call_mod_c"));
}
