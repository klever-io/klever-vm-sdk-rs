use klever_sc::imports::*;
use klever_sc::derive_imports::*;

use crate::crowdfunding_data::CrowdfundingData;

#[klever_sc::module]
pub trait Storage {
    #[storage_mapper("profitAddress")]
    fn profit_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getServiceFee)]
    #[storage_mapper("fee")]
    fn service_fee(&self) -> SingleValueMapper<u32>;

    #[view(getLimit)]
    #[storage_mapper("limit")]
    fn limit(&self) -> SingleValueMapper<u32>;


    #[view(getClaimFee)]
    #[storage_mapper("claimFee")]
    fn claim_fee(&self) -> SingleValueMapper<BigUint>;

    #[view(getCrowdfunding)]
    #[storage_mapper("crowdfunding")]
    fn crowdfunding(
        &self,
        id: &ManagedBuffer,
    ) -> SingleValueMapper<CrowdfundingData<Self::Api>>;

    #[storage_mapper("crowdfundingHoldersValues")]
    fn crowdfundings_holders_values(&self, address: &ManagedAddress) -> VecMapper<ManagedBuffer>;

    #[storage_mapper("crowdfundingsHolders")]
    fn crowdfundings_holders(&self) -> VecMapper<ManagedAddress>;

    #[storage_mapper("lastCrowdfundings")]
    fn crowdfundings(&self) -> VecMapper<ManagedBuffer>;
}