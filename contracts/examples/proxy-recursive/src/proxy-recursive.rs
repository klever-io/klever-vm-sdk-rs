#![no_std]

mod callee_proxy {

    #[klever_sc::proxy]
    pub trait CalleeContract {
        #[endpoint(compute)]
        fn compute(&self, amount: BigUint) -> BigUint;
    }
}

use klever_sc::imports::*;
#[klever_sc::contract]
pub trait ProxyRecursive {
    #[init]
    fn init(&self) {
        self.contract_address()
            .set(self.blockchain().get_sc_address());
    }

    #[storage_mapper("contract_address")]
    fn contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[proxy]
    fn contract_proxy(&self, to: ManagedAddress) -> callee_proxy::Proxy<Self::Api>;

    #[endpoint]
    fn compute(&self, amount: BigUint) -> BigUint {
        if amount == 0 {
            return amount;
        }
        let contract_address = self.contract_address().get();

        let result: BigUint = self
            .contract_proxy(contract_address)
            .compute(amount.clone() - 1u64)
            .execute_on_dest_context();

        return amount + result;
    }
}
