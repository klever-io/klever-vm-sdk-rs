#![no_std]

use klever_sc::imports::*;

#[klever_sc::contract]
pub trait Instance {
    #[view(getInstance)]
    #[storage_mapper("instance")]
    fn instance(&self) -> SingleValueMapper<BigUint>;

    #[view(getReview)]
    #[storage_mapper("review")]
    fn review(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, instance: BigUint) {
        self.instance().set(instance);
        self.review().set(BigUint::from(0u32));
    }

    // Changes review to a new value on contract upgrades
    #[upgrade]
    fn upgrade(&self) {
        // load current value
        let current_value = self.review().get();
        // increment it
        let new_value = current_value + BigUint::from(1u32);
        // set new value
        self.review().set(new_value);
    }
}
