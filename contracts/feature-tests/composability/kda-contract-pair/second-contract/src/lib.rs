#![no_std]

use klever_sc::imports::*;

#[klever_sc::contract]
pub trait SecondContract {
    #[init]
    fn init(&self, kda_token_identifier: TokenIdentifier) {
        self.set_contract_kda_token_identifier(&kda_token_identifier);
    }

    #[payable("*")]
    #[endpoint(acceptKdaPayment)]
    fn accept_kda_payment(&self) {
        let actual_token_identifier = self.call_value().klv_or_single_kda().token_identifier;
        let expected_token_identifier = self.get_contract_kda_token_identifier();
        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong kda token"
        );
    }

    #[payable("*")]
    #[endpoint(rejectKdaPayment)]
    fn reject_kda_payment(&self) {
        sc_panic!("Rejected")
    }

    // storage

    #[storage_set("kdaTokenName")]
    fn set_contract_kda_token_identifier(&self, kda_token_identifier: &TokenIdentifier);

    #[view(getkdaTokenName)]
    #[storage_get("kdaTokenName")]
    fn get_contract_kda_token_identifier(&self) -> TokenIdentifier;
}
