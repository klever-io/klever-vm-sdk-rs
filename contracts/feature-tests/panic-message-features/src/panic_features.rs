#![no_std]

use klever_sc::imports::*;

/// Explores panic messaging.
/// Sending panic messages to the VM is possible, as shown in this contract,
/// but it greatly inflates the bytecode size.
#[klever_sc::contract]
pub trait PanicMessageFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint(panicWithMessage)]
    fn panic_with_message(&self, some_value: u32) {
        panic!("example panic message {some_value}");
    }

    /// Logs do not get recorded in case of panic.
    #[endpoint(panicAfterLog)]
    fn panic_after_log(&self) {
        self.before_panic();
        panic!("panic after log");
    }

    #[event("before-panic")]
    fn before_panic(&self);

    #[view]
    fn sc_panic(&self) {
        sc_panic!("sc_panic! test");
    }
}
