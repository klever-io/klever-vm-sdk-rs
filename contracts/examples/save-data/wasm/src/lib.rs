// Code generated by the klever-sc build system. DO NOT EDIT.
////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Total number of exported functions:   3

#![no_std]

klever_sc_wasm_adapter::allocator!();
klever_sc_wasm_adapter::panic_handler!();

klever_sc_wasm_adapter::endpoints! {
    save_data
    (
        init => init
        save_data => save_data
        getData => data_info
    )
}
