// Code generated by the klever-sc build system. DO NOT EDIT.
////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Total number of exported functions:   8

#![no_std]

klever_sc_wasm_adapter::allocator!(static64k);
klever_sc_wasm_adapter::panic_handler!();

klever_sc_wasm_adapter::endpoints! {
    first_contract
    (
        init => init
        transferToSecondContractFull => transfer_to_second_contract_full
        transferToSecondContractHalf => transfer_to_second_contract_half
        transferToSecondContractRejected => transfer_to_second_contract_rejected
        transferToSecondContractRejectedWithTransferAndExecute => transfer_to_second_contract_rejected_with_transfer_and_execute
        transferToSecondContractFullWithTransferAndExecute => transfer_to_second_contract_full_with_transfer_and_execute
        getkdaTokenName => get_contract_kda_token_identifier
        getSecondContractAddress => get_second_contract_address
    )
}
