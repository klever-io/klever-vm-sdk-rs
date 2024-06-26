// Code generated by the klever-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Total number of exported functions:   3

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

klever_sc_wasm_adapter::allocator!();
klever_sc_wasm_adapter::panic_handler_with_message!();

klever_sc_wasm_adapter::endpoints! {
    panic_message_features
    (
        init => init
        panicWithMessage => panic_with_message
        panicAfterLog => panic_after_log
    )
}
