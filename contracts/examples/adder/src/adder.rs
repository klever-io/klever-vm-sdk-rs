#![no_std]

klever_sc::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[klever_sc::contract]
pub trait Adder {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }

    /// Add desired amount to the storage variable accepting a payment (usage example of payable).
    #[endpoint]
    #[payable("KLV")]
    fn add_payable(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }

    // Changes sum to a new value on contract upgrades
    #[upgrade]
    fn upgrade(&self, new_value: BigUint) {
        self.sum().set(new_value);
    }
}
