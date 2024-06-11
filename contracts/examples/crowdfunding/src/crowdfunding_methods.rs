use klever_sc::imports::*;
use klever_sc::derive_imports::*;

use crate::storage;
use crate::crowdfunding_data::CrowdfundingData;
use crate::crowdfunding_helpers::{name_to_id, discount_percentage_fee};


#[klever_sc::module]
pub trait CrowdfundingMethods: storage::Storage 
{
    #[payable("KLV")]
    #[endpoint]
    fn claim(&self, id: ManagedBuffer) {
       let (token_identifier, payment) = self.call_value().klv_or_single_fungible_kda();

        require!(token_identifier == TokenIdentifier::from("KLV"), "Invalid token for claim!");
        require!(payment == self.claim_fee().get(), "Invalid Fee for claim!");
        require!(!self.crowdfunding(&id).is_empty(),"Crowdfunding does not exists!");

        let mapper = self.crowdfunding(&id);
        let mut crowdfunding = mapper.get();

        require!(crowdfunding.owner == self.blockchain().get_caller(), "You are not the owner!");

        let available = crowdfunding.balance.clone().sub(crowdfunding.claimed.clone());

        require!(available > BigUint::from(0u32), "Claim not available!");

        let fee = self.service_fee().get();

        let claim_value = discount_percentage_fee(&available, fee);

        self.send().direct_kda(&crowdfunding.owner, &crowdfunding.token, 0, &claim_value);

        crowdfunding.claimed += &available;

        let profit = available.clone().sub(&claim_value);

        self.send().direct_kda(
            &self.profit_address().get(),
            &crowdfunding.token,
            0,
            &profit,
        );

        mapper.set(crowdfunding);
    }

    #[endpoint]
    fn create(&self, name: ManagedBuffer, logo: ManagedBuffer, description: ManagedBuffer, 
        token: TokenIdentifier, target: BigUint, deadline: u64) {
        let id = name_to_id(&name);

        require!(id.len() > 3, "Invalid name!");
        require!(deadline > self.blockchain().get_block_timestamp(), "Invalid deadline!");
        require!(target > BigUint::default(), "Invalid target!");
        require!(self.crowdfunding(&id).is_empty(), "Crowdfunding already exists!");

        let caller = &self.blockchain().get_caller();

        let crowdfunding = CrowdfundingData{
            id: id.clone(),
            title: name.clone(),
            logo: logo.clone(),
            description: description.clone(),
            token: token.clone(),
            owner: caller.clone(),
            balance: BigUint::default(),
            claimed: BigUint::default(),
            target: target.clone(),
            donators: 0,
            deadline,
        };

        self.crowdfunding(&id).set(crowdfunding);

        if self.crowdfundings_holders_values(caller).is_empty() {
            self.crowdfundings_holders().push(caller);
        }

        self.crowdfundings_holders_values(caller).push(&id);

        let limit = self.limit().get() as usize;

        if self.crowdfundings().len() < limit {
            self.crowdfundings().push(&id);
            return;
        }

        for (i, _) in self.crowdfundings().iter().enumerate() {
            if i == 0 {
               continue;
            }

            self.crowdfundings().set(i, &self.crowdfundings().get(i+1))
        }

        self.crowdfundings().set(limit, &id);

    }

    
    #[endpoint]
    fn set_deadline(&self, crowdfunding_id: ManagedBuffer, deadline: u64) {
        let mapper = self.crowdfunding(&crowdfunding_id);
        let mut crowdfunding = mapper.get();

        let timestamp = self.blockchain().get_block_timestamp();
        
        require!(deadline > timestamp, "Deadline already passed!");

        crowdfunding.deadline = deadline;

        mapper.set(crowdfunding);
    }

    #[endpoint]
    #[payable("*")]
    fn donate(&self, crowdfunding_id: ManagedBuffer) {
        let (token_identifier, payment) = self.call_value().klv_or_single_fungible_kda();

        require!(!self.crowdfunding(&crowdfunding_id).is_empty(),"Crowdfunding not found!");

        let mapper = self.crowdfunding(&crowdfunding_id);
        let mut crowdfunding = mapper.get();

        require!(
            token_identifier == crowdfunding.token,
            "Invalid token for Crowdfunding!"
        );
        
        crowdfunding.balance += payment;
        crowdfunding.donators += 1;

        mapper.set(crowdfunding);
    }
}
