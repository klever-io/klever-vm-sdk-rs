use klever_sc::imports::*;
use klever_sc::derive_imports::*;

use crate::{storage, crowdfunding_data::CrowdfundingData};

#[klever_sc::module]
pub trait CrowdfundingViews: storage::Storage {
    #[view(getLastCrowdfundings)]
    fn get_last_crowdfundings(&self) -> ManagedVec<CrowdfundingData<Self::Api>> {
        let mut vec = ManagedVec::new();
        let ids = self.crowdfundings();

        for id in &ids {
            let crowdfunding = self.crowdfunding(&id).get();
            vec.push(crowdfunding);
        }
        
        vec
    }

    #[view(getCrowdfundingHolders)]
    fn get_crowdfunding_holders(&self) -> ManagedVec<ManagedAddress> {
        let mut vec = ManagedVec::new();
        let holders = self.crowdfundings_holders();

        for holder in &holders {
            vec.push(holder.clone());
        }

        vec
    }

    #[view(getCrowdfundingsByHolder)]
    fn get_crowdfundings_by_holder(&self, address: &ManagedAddress) -> ManagedVec<CrowdfundingData<Self::Api>>{
        let mut vec = ManagedVec::new();
        let ids = self.crowdfundings_holders_values(address);

        for id in &ids {
            let crowdfunding = self.crowdfunding(&id).get();
            vec.push(crowdfunding);
        }

        vec
    }
}