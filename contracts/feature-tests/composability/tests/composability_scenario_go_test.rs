use klever_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
#[ignore = "waiting for VM 1.5"]
fn builtin_func_delete_user_name_go() {
    world().run("scenarios/builtin_func_delete_user_name.scen.json");
}

#[test]
fn builtin_func_set_user_name_go() {
    world().run("scenarios/builtin_func_set_user_name.scen.json");
}

#[test]
fn forw_raw_init_sync_accept_klv_go() {
    world().run("scenarios/forw_raw_init_sync_accept_klv.scen.json");
}

#[test]
fn forw_raw_init_sync_echo_go() {
    world().run("scenarios/forw_raw_init_sync_echo.scen.json");
}

#[test]
fn forw_raw_sync_echo_go() {
    world().run("scenarios/forw_raw_sync_echo.scen.json");
}

#[test]
fn forw_raw_sync_echo_caller_go() {
    world().run("scenarios/forw_raw_sync_echo_caller.scen.json");
}

#[test]
fn forw_raw_sync_klv_go() {
    world().run("scenarios/forw_raw_sync_klv.scen.json");
}

#[test]
fn forw_raw_sync_readonly_go() {
    world().run("scenarios/forw_raw_sync_readonly.scen.json");
}

#[test]
fn forw_raw_sync_same_context_go() {
    world().run("scenarios/forw_raw_sync_same_context.scen.json");
}

#[test]
fn forw_raw_sync_same_context_klv_go() {
    world().run("scenarios/forw_raw_sync_same_context_klv.scen.json");
}

#[test]
fn forw_raw_transf_exec_accept_klv_go() {
    world().run("scenarios/forw_raw_transf_exec_accept_klv.scen.json");
}

#[test]
fn forw_raw_transf_exec_reject_klv_go() {
    world().run("scenarios/forw_raw_transf_exec_reject_klv.scen.json");
}

#[test]
fn forwarder_builtin_nft_add_quantity_go() {
    world().run("scenarios/forwarder_builtin_nft_add_quantity.scen.json");
}

#[test]
fn forwarder_builtin_nft_burn_go() {
    world().run("scenarios/forwarder_builtin_nft_burn.scen.json");
}

#[test]
fn forwarder_builtin_nft_create_go() {
    world().run("scenarios/forwarder_builtin_nft_create.scen.json");
}

#[test]
fn forwarder_builtin_nft_local_burn_go() {
    world().run("scenarios/forwarder_builtin_nft_local_burn.scen.json");
}

#[test]
fn forwarder_builtin_nft_local_mint_go() {
    world().run("scenarios/forwarder_builtin_nft_local_mint.scen.json");
}

#[test]
fn forwarder_call_sync_accept_kda_go() {
    world().run("scenarios/forwarder_call_sync_accept_kda.scen.json");
}

#[test]
fn forwarder_call_sync_accept_klv_go() {
    world().run("scenarios/forwarder_call_sync_accept_klv.scen.json");
}

#[test]
fn forwarder_call_sync_accept_multi_transfer_go() {
    world().run("scenarios/forwarder_call_sync_accept_multi_transfer.scen.json");
}

#[test]
fn forwarder_call_sync_accept_nft_go() {
    world().run("scenarios/forwarder_call_sync_accept_nft.scen.json");
}

#[test]
fn forwarder_call_sync_accept_then_read_kda_go() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_kda.scen.json");
}

#[test]
fn forwarder_call_sync_accept_then_read_klv_go() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_klv.scen.json");
}

#[test]
fn forwarder_call_sync_accept_then_read_nft_go() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_nft.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_kda_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_kda.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_klv_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_klv.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_nft_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_nft.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_kda_go() {
    world().run("scenarios/forwarder_call_transf_exec_accept_kda.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_kda_twice_go() {
    world().run("scenarios/forwarder_call_transf_exec_accept_kda_twice.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_klv_go() {
    world().run("scenarios/forwarder_call_transf_exec_accept_klv.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_klv_twice_go() {
    world().run("scenarios/forwarder_call_transf_exec_accept_klv_twice.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_multi_transfer_go() {
    world().run("scenarios/forwarder_call_transf_exec_accept_multi_transfer.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_nft_go() {
    world().run("scenarios/forwarder_call_transf_exec_accept_nft.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_return_values_go() {
    world().run("scenarios/forwarder_call_transf_exec_accept_return_values.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_sft_twice_go() {
    world().run("scenarios/forwarder_call_transf_exec_accept_sft_twice.scen.json");
}

#[test]
fn forwarder_call_transf_exec_reject_multi_transfer_go() {
    world().run("scenarios/forwarder_call_transf_exec_reject_multi_transfer.scen.json");
}

#[test]
fn forwarder_call_transf_exec_reject_nft_go() {
    world().run("scenarios/forwarder_call_transf_exec_reject_nft.scen.json");
}

#[test]
fn forwarder_contract_change_owner_go() {
    world().run("scenarios/forwarder_contract_change_owner.scen.json");
}

#[test]
fn forwarder_contract_deploy_go() {
    world().run("scenarios/forwarder_contract_deploy.scen.json");
}

#[test]
fn forwarder_contract_upgrade_go() {
    world().run("scenarios/forwarder_contract_upgrade.scen.json");
}

#[test]
fn forwarder_get_kda_local_roles_go() {
    world().run("scenarios/forwarder_get_kda_local_roles.scen.json");
}

#[test]
fn forwarder_get_kda_token_data_go() {
    world().run("scenarios/forwarder_get_kda_token_data.scen.json");
}

#[test]
fn forwarder_nft_add_uri_go() {
    world().run("scenarios/forwarder_nft_add_uri.scen.json");
}

#[test]
fn forwarder_nft_create_go() {
    world().run("scenarios/forwarder_nft_create.scen.json");
}

#[test]
fn forwarder_nft_create_and_send_go() {
    world().run("scenarios/forwarder_nft_create_and_send.scen.json");
}

#[test]
fn forwarder_nft_current_nonce_go() {
    world().run("scenarios/forwarder_nft_current_nonce.scen.json");
}

#[test]
fn forwarder_nft_decode_complex_attributes_go() {
    world().run("scenarios/forwarder_nft_decode_complex_attributes.scen.json");
}

#[test]
fn forwarder_nft_transfer_exec_go() {
    world().run("scenarios/forwarder_nft_transfer_exec.scen.json");
}

#[test]
fn forwarder_nft_update_attributes_go() {
    world().run("scenarios/forwarder_nft_update_attributes.scen.json");
}

#[test]
fn forwarder_no_endpoint_go() {
    world().run("scenarios/forwarder_no_endpoint.scen.json");
}

#[test]
fn forwarder_retrieve_funds_with_accept_func_go() {
    world().run("scenarios/forwarder_retrieve_funds_with_accept_func.scen.json");
}

#[test]
fn forwarder_send_kda_multi_transfer_go() {
    world().run("scenarios/forwarder_send_kda_multi_transfer.scen.json");
}

#[test]
fn forwarder_send_twice_kda_go() {
    world().run("scenarios/forwarder_send_twice_kda.scen.json");
}

#[test]
fn forwarder_send_twice_klv_go() {
    world().run("scenarios/forwarder_send_twice_klv.scen.json");
}

#[test]
fn forwarder_sync_echo_go() {
    world().run("scenarios/forwarder_sync_echo.scen.json");
}

#[test]
fn forwarder_tranfer_kda_with_fees_go() {
    world().run("scenarios/forwarder_tranfer_kda_with_fees.scen.json");
}

#[test]
fn forwarder_validate_token_identifier_go() {
    world().run("scenarios/forwarder_validate_token_identifier.scen.json");
}

#[test]
fn proxy_test_init_go() {
    world().run("scenarios/proxy_test_init.scen.json");
}

#[test]
fn proxy_test_message_other_shard_go() {
    world().run("scenarios/proxy_test_message_otherShard.scen.json");
}

#[test]
fn proxy_test_message_other_shard_callback_go() {
    world().run("scenarios/proxy_test_message_otherShard_callback.scen.json");
}

#[test]
fn proxy_test_message_same_shard_go() {
    world().run("scenarios/proxy_test_message_sameShard.scen.json");
}

#[test]
fn proxy_test_message_same_shard_callback_go() {
    world().run("scenarios/proxy_test_message_sameShard_callback.scen.json");
}

#[test]
fn proxy_test_payment_other_shard_go() {
    world().run("scenarios/proxy_test_payment_otherShard.scen.json");
}

#[test]
fn proxy_test_payment_other_shard_callback_go() {
    world().run("scenarios/proxy_test_payment_otherShard_callback.scen.json");
}

#[test]
fn proxy_test_payment_same_shard_go() {
    world().run("scenarios/proxy_test_payment_sameShard.scen.json");
}

#[test]
fn proxy_test_payment_same_shard_callback_go() {
    world().run("scenarios/proxy_test_payment_sameShard_callback.scen.json");
}

#[test]
fn proxy_test_upgrade_go() {
    world().run("scenarios/proxy_test_upgrade.scen.json");
}

#[test]
fn recursive_caller_kda_1_go() {
    world().run("scenarios/recursive_caller_kda_1.scen.json");
}

#[test]
fn recursive_caller_klv_1_go() {
    world().run("scenarios/recursive_caller_klv_1.scen.json");
}

#[test]
fn send_kda_go() {
    world().run("scenarios/send_kda.scen.json");
}

#[test]
fn send_klv_go() {
    world().run("scenarios/send_klv.scen.json");
}
