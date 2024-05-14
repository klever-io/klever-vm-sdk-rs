use klever_sc::api::KLEVER_TRANSFER_FUNC_NAME;
use klever_sc_scenario::scenario_model::ScCallStep;
use num_traits::Zero;

#[test]
fn test_contract_call_multi_kda() {
    let tx = ScCallStep::new()
        .from("address:sender")
        .to("address:recipient")
        .kda_transfer("str:WKLV-abcdef", 0, 10u32)
        .kda_transfer("str:USDC-abcdef", 0, 11u32);

    let cc = tx.tx.to_contract_call();

    assert_eq!(
        cc.basic.function_call.function_name.to_vec(),
        KLEVER_TRANSFER_FUNC_NAME.as_bytes().to_vec(),
    );
    assert_eq!(
        cc.to_call_data_string().to_string(),
        "KleverTransfer@726563697069656e745f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f@02@574b4c562d616263646566@@0a@555344432d616263646566@@0b",
    );

    assert!(tx.tx.klv_value.value.is_zero());
}
