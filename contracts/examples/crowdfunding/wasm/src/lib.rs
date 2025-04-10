// Code generated by the klever-sc build system. DO NOT EDIT.
////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           15
// Total number of exported functions:  16

#![no_std]

klever_sc_wasm_adapter::allocator!();
klever_sc_wasm_adapter::panic_handler!();

klever_sc_wasm_adapter::endpoints! {
    crowdfunding
    (
        init => init
        claim => claim
        create => create
        set_deadline => set_deadline
        donate => donate
        set_profit_address => set_profit_address
        set_fee => set_fee
        set_limit => set_limit
        set_claim_fee => set_claim_fee
        getLastCrowdfundings => get_last_crowdfundings
        getCrowdfundingHolders => get_crowdfunding_holders
        getCrowdfundingsByHolder => get_crowdfundings_by_holder
        getServiceFee => service_fee
        getLimit => limit
        getClaimFee => claim_fee
        getCrowdfunding => crowdfunding
    )
}
