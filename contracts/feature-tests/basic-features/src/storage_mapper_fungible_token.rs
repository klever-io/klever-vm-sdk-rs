klever_sc::imports!();

#[klever_sc::module]
pub trait FungibleTokenMapperFeatures {
    #[payable("KLV")]
    #[endpoint]
    fn issue_fungible_default_callback(
        &self,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        self.fungible_token_mapper().issue(
            &ManagedBuffer::new(),
            &token_ticker,
            &initial_supply,
            &initial_supply,
            0,
        );
    }

    #[payable("KLV")]
    #[endpoint]
    fn issue_fungible_custom_callback(&self, token_ticker: ManagedBuffer, initial_supply: BigUint) {
        let token_identifier = self.fungible_token_mapper().issue(
            &ManagedBuffer::new(),
            &token_ticker,
            &initial_supply,
            &initial_supply,
            0,
        );

        self.fungible_token_mapper().set_token_id(token_identifier);
    }

    #[endpoint]
    fn mint_fungible(&self, amount: BigUint) -> KdaTokenPayment<Self::Api> {
        self.fungible_token_mapper().mint(&amount)
    }

    #[endpoint]
    fn mint_and_send_fungible(
        &self,
        to: ManagedAddress,
        amount: BigUint,
    ) -> KdaTokenPayment<Self::Api> {
        self.fungible_token_mapper().mint_and_send(&to, &amount)
    }

    #[endpoint]
    fn burn_fungible(&self, amount: BigUint) {
        self.fungible_token_mapper().burn(&amount);
    }

    #[endpoint]
    fn get_balance_fungible(&self) -> BigUint {
        self.fungible_token_mapper().get_balance()
    }

    #[payable("*")]
    #[endpoint]
    fn require_same_token_fungible(&self) {
        let payment_token = self.call_value().single_kda().token_identifier;
        self.fungible_token_mapper()
            .require_same_token(&payment_token);
    }

    #[payable("*")]
    #[endpoint]
    fn require_all_same_token_fungible(&self) {
        let payments = self.call_value().all_kda_transfers();
        self.fungible_token_mapper()
            .require_all_same_token(&payments);
    }

    #[view(getFungibleTokenId)]
    #[storage_mapper("fungibleTokenMapper")]
    fn fungible_token_mapper(&self) -> FungibleTokenMapper;

    #[storage_mapper("rolesSet")]
    fn roles_set(&self) -> SingleValueMapper<bool>;
}
