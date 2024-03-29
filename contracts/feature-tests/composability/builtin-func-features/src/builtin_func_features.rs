#![no_std]

pub mod builtin_func_proxy;

klever_sc::imports!();

/// Test contract for investigating calls.
#[klever_sc::contract]
pub trait BuiltinFuncFeatures {
    #[proxy]
    fn builtin_func_proxy(&self, to: ManagedAddress) -> builtin_func_proxy::Proxy<Self::Api>;

    #[init]
    fn init(&self) {}

    #[endpoint]
    fn call_set_user_name(&self, address: ManagedAddress, name: ManagedBuffer) {
        self.builtin_func_proxy(address)
            .set_user_name(&name)
            .execute_on_dest_context::<IgnoreValue>();
    }

    #[endpoint]
    fn call_delete_user_name(&self, address: ManagedAddress) {
        self.builtin_func_proxy(address)
            .delete_user_name()
            .execute_on_dest_context::<IgnoreValue>();
    }
}
