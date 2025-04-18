// Code generated by the klever-sc build system. DO NOT EDIT.
////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Total number of exported functions:   9

#![no_std]

klever_sc_wasm_adapter::allocator!();
klever_sc_wasm_adapter::panic_handler!();

klever_sc_wasm_adapter::endpoints! {
    proxy_pause
    (
        init => init
        addContracts => add_contracts
        removeContracts => remove_contracts
        addOwners => add_owners
        removeOwners => remove_owners
        pause => pause
        unpause => unpause
        owners => owners
        contracts => contracts
    )
}
