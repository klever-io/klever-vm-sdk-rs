// Code generated by the klever-sc build system. DO NOT EDIT.
////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            4
// Total number of exported functions:   5

#![no_std]

klever_sc_wasm_adapter::allocator!();
klever_sc_wasm_adapter::panic_handler!();

klever_sc_wasm_adapter::endpoints! {
    check_pause
    (
        init => init
        checkPause => check_pause
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}
