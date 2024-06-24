#![no_std]

use klever_sc::imports::*;


#[klever_sc::contract]
pub trait DepositKDAPoolContract {
    #[init]
    fn init(&self, pool_id: TokenIdentifier) {
        self.pool_id().set(pool_id);
    }

    #[view(getPoolID)]
    #[storage_mapper("poolID")]
    fn pool_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[payable("*")]
    #[endpoint(deposit)]
    fn deposit(&self) {

        let (token_identifier, payment) = self.call_value().klv_or_single_fungible_kda();
        require!(
            token_identifier == TokenIdentifier::from("KLV") ||  token_identifier == self.pool_id().get(),
            "payment token must be in KLV or defined pool ID"
        );

        require!(payment > 0, "payment amount can't be zero");

        self.send().deposit_kda_pool(&self.pool_id().get().into_managed_buffer(), &token_identifier, &BigUint::from(payment))
    }
}
