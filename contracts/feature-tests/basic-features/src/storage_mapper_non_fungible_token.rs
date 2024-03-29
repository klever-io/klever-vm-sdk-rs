klever_sc::imports!();
klever_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[klever_sc::module]
pub trait NonFungibleTokenMapperFeatures {
    #[payable("KLV")]
    #[endpoint]
    fn issue(&self, token_ticker: ManagedBuffer) {
        self.non_fungible_token_mapper().issue(     
            &ManagedBuffer::new(),
            &token_ticker,
        );
    }

    #[endpoint]
    fn mapper_nft_set_token_id(&self, token_id: TokenIdentifier) {
        self.non_fungible_token_mapper().set_token_id(token_id);
    }

    #[endpoint]
    fn mapper_nft_mint(
        &self,
        amount: BigUint,
    ) {
        self.non_fungible_token_mapper()
            .nft_mint(&amount);
    }

    #[endpoint]
    fn mapper_nft_burn(&self, token_nonce: u64, amount: BigUint) {
        self.non_fungible_token_mapper()
            .nft_burn(token_nonce,  &amount);
    }

    #[endpoint]
    fn mapper_nft_get_balance(&self, token_nonce: u64) -> BigUint {
        self.non_fungible_token_mapper().get_balance(token_nonce)
    }

    #[endpoint]
    fn mapper_get_token_attributes(&self, token_nonce: u64) -> RgbColor {
        self.non_fungible_token_mapper()
            .get_token_attributes(token_nonce)
    }

    #[view(getNonFungibleTokenId)]
    #[storage_mapper("nonFungibleTokenMapper")]
    fn non_fungible_token_mapper(&self) -> NonFungibleTokenMapper;
}
