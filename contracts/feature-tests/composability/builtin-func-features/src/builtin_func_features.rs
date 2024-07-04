#![no_std]

pub mod builtin_func_proxy;

use klever_sc::imports::*;

/// Test contract for investigating calls.
#[klever_sc::contract]
pub trait BuiltinFuncFeatures {

    #[init]
    fn init(&self) {}

    #[endpoint]
    fn call_set_account_name(&self, address: ManagedAddress, name: ManagedBuffer) {
        self.tx()
            .to(&address)
            .typed(builtin_func_proxy::UserBuiltinProxy)
            .set_user_name(name)
            .execute_on_dest_context()
    }
}
