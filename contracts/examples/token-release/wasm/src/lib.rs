// Code generated by the klever-sc build system. DO NOT EDIT.
////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           13
// Total number of exported functions:  14

#![no_std]

klever_sc_wasm_adapter::allocator!();
klever_sc_wasm_adapter::panic_handler!();

klever_sc_wasm_adapter::endpoints! {
    token_release
    (
        init => init
        addFixedAmountGroup => add_fixed_amount_group
        addPercentageBasedGroup => add_percentage_based_group
        removeGroup => remove_group
        addUserGroup => add_user_group
        removeUser => remove_user
        requestAddressChange => request_address_change
        approveAddressChange => approve_address_change
        endSetupPeriod => end_setup_period
        claimTokens => claim_tokens
        verify_address_change => verify_address_change
        get_claimable_tokens => get_claimable_tokens
        getTokenIdentifier => token_identifier
        getTokenTotalSupply => token_total_supply
    )
}
