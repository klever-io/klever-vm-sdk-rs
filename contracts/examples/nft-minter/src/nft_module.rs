use klever_sc::derive_imports::*;
use klever_sc::imports::*;

const NFT_AMOUNT: u32 = 1;
const ROYALTIES_MAX: u32 = 10_000;

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

        self.send().kda_system_sc_proxy().issue_non_fungible(
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

        self.send().kda_system_sc_proxy().set_special_roles(
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
        self.tx()
            .to(ToCaller)
            .single_kda(&nft_token_id, nft_nonce, &BigUint::from(NFT_AMOUNT))
            .transfer();

        let owner = self.blockchain().get_owner_address();
        self.tx().to(owner).payment(payment).transfer();
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

    #[allow(clippy::too_many_arguments)]
    fn mint_nft<T: TopEncode>(
        &self,
        name: ManagedBuffer,
        royalties: BigUint,
        attributes: T,
        _uri: ManagedBuffer,
        selling_price: BigUint,
        token_used_as_payment: TokenIdentifier,
        token_used_as_payment_nonce: u64,
    ) -> u64 {
        self.require_token_issued();
        require!(royalties <= ROYALTIES_MAX, "Royalties cannot exceed 100%");

        let nft_token_id = self.nft_token_id().get();

        let result = self
            .send()
            .kda_mint(&nft_token_id, 0, &BigUint::from(NFT_AMOUNT));

        // get nonce of the minted NFT
        let nft_nonce = if let Some(first_result_bytes) = result.try_get(0) {
            first_result_bytes.parse_as_u64().unwrap()
        } else {
            panic!("Failed to get nonce from mint result")
        };

        let mut serialized_attributes = ManagedBuffer::new();
        if let core::result::Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }

        // update metadata
        self.send().kda_system_sc_proxy().update_metadata(
            &nft_token_id,
            nft_nonce,
            &self.blockchain().get_sc_address(),
            &ManagedBuffer::new(),
            &serialized_attributes,
            &name,
        );

        self.price_tag(nft_nonce).set(&PriceTag {
            token: token_used_as_payment,
            nonce: token_used_as_payment_nonce,
            amount: selling_price,
        });

        nft_nonce
    }

    // storage

    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("priceTag")]
    fn price_tag(&self, nft_nonce: u64) -> SingleValueMapper<PriceTag<Self::Api>>;
}
