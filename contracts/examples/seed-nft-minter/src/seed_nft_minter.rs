#![no_std]

use klever_sc::imports::*;
use klever_sc::derive_imports::*;

mod distribution_module;
mod nft_module;

use distribution_module::Distribution;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

#[klever_sc::contract]
pub trait SeedNftMinter:
    distribution_module::DistributionModule
    + nft_module::NftModule
{
    #[init]
    fn init(
        &self,
        marketplaces: ManagedVec<ManagedAddress>,
        distribution: ManagedVec<Distribution<Self::Api>>,
    ) {
        self.marketplaces().extend(&marketplaces);
        self.init_distribution(distribution);
    }

    #[only_owner]
    #[endpoint(claimAndDistribute)]
    fn claim_and_distribute(&self, token_id: TokenIdentifier, token_nonce: u64) {
        let total_amount = self.claim_royalties(&token_id, token_nonce);
        self.distribute_funds(&token_id, token_nonce, total_amount);
    }

    fn claim_royalties(&self, token_id: &TokenIdentifier, token_nonce: u64) -> BigUint {
        let claim_destination = self.blockchain().get_sc_address();
        let mut total_amount = BigUint::zero();
        for address in self.marketplaces().iter() {
            let results: MultiValue2<BigUint, ManagedVec<KdaTokenPayment>> = self
                .marketplace_proxy(address)
                .claim_tokens(&claim_destination, token_id, token_nonce)
                .execute_on_dest_context();

            let (klv_amount, kda_payments) = results.into_tuple();
            let amount = if token_id.is_klv() {
                klv_amount
            } else {
                kda_payments
                    .try_get(0)
                    .map(|kda_payment| kda_payment.amount)
                    .unwrap_or_default()
            };
            total_amount += amount;
        }

        total_amount
    }

    #[view(getMarketplaces)]
    #[storage_mapper("marketplaces")]
    fn marketplaces(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getNftCount)]
    #[storage_mapper("nftCount")]
    fn nft_count(&self) -> SingleValueMapper<u64>;

    #[proxy]
    fn marketplace_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> nft_marketplace_proxy::Proxy<Self::Api>;
}

mod nft_marketplace_proxy {
    use klever_sc::imports::*;

    #[klever_sc::proxy]
    pub trait NftMarketplace {
        #[endpoint(claimTokens)]
        fn claim_tokens(
            &self,
            claim_destination: &ManagedAddress,
            token_id: &TokenIdentifier,
            token_nonce: u64,
        ) -> MultiValue2<BigUint, ManagedVec<KdaTokenPayment>>;
    }
}
