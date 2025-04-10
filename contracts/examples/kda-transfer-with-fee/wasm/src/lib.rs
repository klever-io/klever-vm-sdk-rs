// Code generated by the klever-sc build system. DO NOT EDIT.
////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Total number of exported functions:   7

#![no_std]

klever_sc_wasm_adapter::allocator!();
klever_sc_wasm_adapter::panic_handler!();

klever_sc_wasm_adapter::endpoints! {
    kda_transfer_with_fee
    (
        init => init
        setExactValueFee => set_exact_value_fee
        setPercentageFee => set_percentage_fee
        claimFees => claim_fees
        transfer => transfer
        getTokenFee => token_fee
        getPaidFees => paid_fees
    )
}
