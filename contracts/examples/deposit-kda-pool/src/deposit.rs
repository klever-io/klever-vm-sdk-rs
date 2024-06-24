#![no_std]

use klever_sc::imports::*;


#[klever_sc::contract]
pub trait DepositKDAPoolContract {
    #[init]
    fn init(&self) {}

    #[payable("KLV")]
    #[endpoint(deposit)]
    fn deposit(&self) {

        let (token_identifier, payment) = self.call_value().klv_or_single_fungible_kda();
        require!(
            token_identifier == TokenIdentifier::from("KLV"),
            "payment token must be KLV"
        );

        require!(payment > 0, "payment amount can't be zero");

        self.send().deposit_kda_pool(&ManagedBuffer::from("PTS-3UUK"), &TokenIdentifier::from("KLV"),   &BigUint::from(payment))
    }
}
