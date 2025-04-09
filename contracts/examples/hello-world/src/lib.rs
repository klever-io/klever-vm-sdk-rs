#![no_std]

use klever_sc::imports::*;

#[klever_sc::contract]
pub trait HelloWorld: ContractBase {
    #[init]
    fn init(&self) {}

    #[event("message")]
    fn message(&self, msg: &str);

    #[endpoint]
    fn get_message(&self) {
        self.message("Hello World!");
    }

    #[endpoint]
    #[payable("KLV")]
    fn pay_hello(&self) {
        let payment = self.call_value().klv_value();

        require!(
            *payment == BigUint::from(10u8),
            "The payment must be 10 KLV."
        );

        self.message("Hello World!");
    }
}
