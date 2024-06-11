use klever_sc::imports::*;
use klever_sc::derive_imports::*;

const NFT_AMOUNT: u32 = 1;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct PriceTag<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub nonce: u64,
    pub amount: BigUint<M>,
}

#[klever_sc::module]
pub trait NftModule {
    // endpoints - owner-only

    #[only_owner]
    #[payable("KLV")]
    #[endpoint(issueToken)]
    fn issue_token(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        require!(self.nft_token_id().is_empty(), "Token already issued");

        self.send()
            .kda_system_sc_proxy()
            .issue_non_fungible(
                &token_name,
                &token_ticker,
                &PropertiesInfo {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: false,
                    can_add_roles: true,
                    limit_transfer: false,
                },
            );
    }

    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self, address: ManagedAddress) {
        self.require_token_issued();

        self.send()
            .kda_system_sc_proxy()
            .set_special_roles(
                &address,
                &self.nft_token_id().get(),
                true,
                false,
                false,
                true,
            )
    }

    // endpoints

    #[payable("*")]
    #[endpoint(buyNft)]
    fn buy_nft(&self, nft_nonce: u64) {
        let payment = self.call_value().klv_or_single_kda();

        self.require_token_issued();
        require!(
            !self.price_tag(nft_nonce).is_empty(),
            "Invalid nonce or NFT was already sold"
        );

        let price_tag = self.price_tag(nft_nonce).get();
        require!(
            payment.token_identifier == price_tag.token,
            "Invalid token used as payment"
        );
        require!(
            payment.token_nonce == price_tag.nonce,
            "Invalid nonce for payment token"
        );
        require!(
            payment.amount == price_tag.amount,
            "Invalid amount as payment"
        );

        self.price_tag(nft_nonce).clear();

        let nft_token_id = self.nft_token_id().get();
        let caller = self.blockchain().get_caller();
        self.send().direct_kda(
            &caller,
            &nft_token_id,
            nft_nonce,
            &BigUint::from(NFT_AMOUNT),
        );

        let owner = self.blockchain().get_owner_address();
        self.send().direct_kda(
            &owner,
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
    }

    // views

    #[allow(clippy::type_complexity)]
    #[view(getNftPrice)]
    fn get_nft_price(
        &self,
        nft_nonce: u64,
    ) -> OptionalValue<MultiValue3<TokenIdentifier, u64, BigUint>> {
        if self.price_tag(nft_nonce).is_empty() {
            // NFT was already sold
            OptionalValue::None
        } else {
            let price_tag = self.price_tag(nft_nonce).get();

            OptionalValue::Some((price_tag.token, price_tag.nonce, price_tag.amount).into())
        }
    }

    // private

    
    fn require_token_issued(&self) {
        require!(!self.nft_token_id().is_empty(), "Token not issued");
    }

    // storage

    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("priceTag")]
    fn price_tag(&self, nft_nonce: u64) -> SingleValueMapper<PriceTag<Self::Api>>;
}
