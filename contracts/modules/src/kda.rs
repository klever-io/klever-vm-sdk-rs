use klever_sc::imports::*;

/// Standard smart contract module for managing a single KDA.
///
/// When added to a smart contract offers basic KDA usage.
/// A lot of contracts use an owned KDA for various purposes.
/// This module is used to offer a standard way of performing the basic operations.  
///
/// It provides endpoints for:
/// * issuing of an KDA
/// * setting local roles
/// * minting/burning
///
#[klever_sc::module]
pub trait KdaModule {
    /*
        KdaTokenType is an enum (u8):
        0 - Fungible,
        1 - NonFungible,
        2 - SemiFungible

        Note: Only Fungible and SemiFungible have decimals
    */
    #[payable("KLV")]
    #[only_owner]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_type: KdaTokenType,
        initial_supply: BigUint,
        max_supply: BigUint,
        opt_num_decimals: OptionalValue<u32>,
    ) -> TokenIdentifier {
        require!(self.token_id().is_empty(), "Token already issued");

        let precision = match opt_num_decimals {
            OptionalValue::Some(d) => d,
            OptionalValue::None => 0,
        };

        match token_type {
            KdaTokenType::Fungible => self.send().kda_system_sc_proxy().issue_fungible(
                &token_display_name,
                precision,
                &token_ticker,
                &initial_supply,
                &max_supply,
                &PropertiesInfo::default(),
                &AttributesInfo::default(),
                &ManagedVec::<Self::Api, URI<Self::Api>>::new(),
            ),
            KdaTokenType::NonFungible => self.send().kda_system_sc_proxy().issue_non_fungible(
                &token_display_name,
                &token_ticker,
                &PropertiesInfo::default(),
                &AttributesInfo::default(),
                &ManagedVec::<Self::Api, URI<Self::Api>>::new(),
            ),
            KdaTokenType::SemiFungible => self.send().kda_system_sc_proxy().issue_semi_fungible(
                &token_display_name,
                &token_ticker,
                precision,
                &PropertiesInfo::default(),
                &AttributesInfo::default(),
                &ManagedVec::<Self::Api, URI<Self::Api>>::new(),
            ),
            _ => panic!("Invalid token type"),
        }
    }

    fn mint(&self, token_nonce: u64, amount: &BigUint) {
        let token_id = self.token_id().get();
        self.send().kda_mint(&token_id, token_nonce, amount);
    }

    fn burn(&self, token_nonce: u64, amount: &BigUint) {
        let token_id = self.token_id().get();
        self.send().kda_burn(&token_id, token_nonce, amount);
    }

    /// Retrieves and decode SFT token attributes
    fn get_sft_attributes<T: TopDecode>(&self, token_nonce: u64) -> T {
        let token_id = self.token_id().get();
        let meta = self.blockchain().get_sft_metadata(&token_id, token_nonce);

        meta.metadata.decode_attributes()
    }

    fn require_token_issued(&self) {
        require!(!self.token_id().is_empty(), "Token must be issued first");
    }

    // Note: to issue another token, you have to clear this storage
    #[storage_mapper("token_id")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
