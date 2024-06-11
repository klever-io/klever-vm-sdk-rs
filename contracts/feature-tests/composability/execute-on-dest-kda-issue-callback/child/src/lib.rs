#![no_std]

use klever_sc::imports::*;

use klever_sc::api::AssetType;

const KLV_DECIMALS: u32 = 6;

#[klever_sc::contract]
pub trait Child {
    #[init]
    fn init(&self) {}

    #[payable("KLV")]
    #[endpoint(issueWrappedKlv)]
    fn issue_wrapped_klv(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let max_supply = initial_supply.clone().mul(BigUint::from(2u32));
        self.send()
            .kda_create(
                AssetType::Fungible,
                &token_display_name,
                &token_ticker,
                KLV_DECIMALS,
                &ManagedAddress::zero(),
                &ManagedBuffer::new(),
                 &initial_supply,
                 &max_supply,
                 &PropertiesInfo { 
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: false,
                    can_add_roles: true,
                    limit_transfer: false,
                  }, 
                &RoyaltiesData::default(),
            );
    }

    // storage

    #[view(getWrappedKlvTokenIdentifier)]
    #[storage_mapper("wrappedKlvTokenIdentifier")]
    fn wrapped_klv_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;
}
