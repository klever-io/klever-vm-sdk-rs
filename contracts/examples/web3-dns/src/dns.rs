#![no_std]

extern crate alloc;
use klever_sc::imports::*;
use klever_sc::types::heap::String;

#[klever_sc::contract]
pub trait Web3DNS {
    #[init]
    fn init(&self) {}

    #[storage_mapper("dns")]
    fn dns(&self, domain: String, record: String) -> SingleValueMapper<String>;

    #[endpoint]
    #[payable("*")]
    fn register(&self, domain: String, record: String, value: String) {
        let payment = self.call_value().klv_value();

        require!(
            *payment == BigUint::from(10u8),
            "The payment must be 10 KLV."
        );
        self.dns(domain, record).set(value);
    }

    #[view]
    fn get_record(&self, domain: String, record: String) -> String {
        self.dns(domain, record).get()
    }
}
