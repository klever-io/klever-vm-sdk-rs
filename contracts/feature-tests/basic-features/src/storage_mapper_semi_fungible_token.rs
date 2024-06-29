use klever_sc::imports::*;
use klever_sc::derive_imports::*;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[klever_sc::module]
pub trait SemiFungibleTokenMapperFeatures {
    #[payable("KLV")]
    #[endpoint]
    fn issue_sft(&self, token_ticker: ManagedBuffer) {
        self.semi_fungible_token_mapper().issue(     
            &ManagedBuffer::new(),
            &token_ticker,
            Some(0),
        );
    }

    #[endpoint]
    fn mapper_sft_set_token_id(&self, token_id: TokenIdentifier) {
        self.semi_fungible_token_mapper().set_token_id(token_id);
    }

    #[endpoint]
    fn mapper_sft_mint(
        &self,
        amount: BigUint,
        attributes: RgbColor,
    ) -> KdaTokenPayment<Self::Api>  {
        let token_nonce = self.semi_fungible_token_mapper()
            .sft_mint(&amount, &attributes);
        let token_id = self.semi_fungible_token_mapper().get_token_id();

        KdaTokenPayment::new(token_id, token_nonce, amount)
    }

    #[endpoint]
    fn mapper_sft_add_quantity(
        &self,
        token_nonce: u64,
        amount: BigUint,
    ) -> KdaTokenPayment<Self::Api>  {
        self.semi_fungible_token_mapper()
            .sft_add_quantity(token_nonce, &amount)
    }

    #[endpoint]
    fn mapper_sft_mint_and_send(
        &self,
        to: ManagedAddress,
        amount: BigUint,
        attributes: RgbColor,
    ) -> KdaTokenPayment<Self::Api>  {
        let token_nonce = self.semi_fungible_token_mapper()
            .sft_mint(&amount, &attributes);

        let token_id = self.semi_fungible_token_mapper().get_token_id();
        
        self.send().direct_kda(&to, &token_id, token_nonce, &amount);

        KdaTokenPayment::new(token_id, token_nonce, amount)
    }

    #[endpoint]
    fn mapper_sft_add_quantity_and_send(
        &self,
        to: ManagedAddress,
        token_nonce: u64,
        amount: BigUint,
    ) -> KdaTokenPayment<Self::Api> {
        self.semi_fungible_token_mapper()
            .sft_add_quantity_and_send(&to, token_nonce, &amount)
    }

    #[endpoint]
    fn mapper_sft_burn(&self, token_nonce: u64, amount: BigUint) {
        self.semi_fungible_token_mapper()
            .sft_burn(token_nonce,  &amount);
    }

    #[endpoint]
    fn mapper_sft_get_balance(&self, token_nonce: u64) -> BigUint {
        self.semi_fungible_token_mapper().get_balance(token_nonce)
    }

    #[endpoint]
    fn mapper_get_sft_attributes(&self, token_nonce: u64) -> RgbColor {
        self.semi_fungible_token_mapper()
            .get_sft_meta_attributes(token_nonce)
    }

    #[view(getSemiFungibleTokenId)]
    #[storage_mapper("semiFungibleTokenMapper")]
    fn semi_fungible_token_mapper(&self) -> SemiFungibleTokenMapper;
    
}
