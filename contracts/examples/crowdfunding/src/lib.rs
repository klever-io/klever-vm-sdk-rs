#![no_std]

use klever_sc::imports::*;

mod storage;
mod storage_setters;
mod crowdfunding_data;
mod crowdfunding_views;
mod crowdfunding_methods;
pub mod crowdfunding_helpers;

#[klever_sc::contract]
pub trait Crowdfunding:
    crowdfunding_methods::CrowdfundingMethods
    + storage_setters::StorageSetters
    + crowdfunding_views::CrowdfundingViews
    + storage::Storage 
{
    #[init]
    fn init(&self, service_fee: u32, limit: u32, profit_address: ManagedAddress, claim_fee: BigUint) {
        self.service_fee().set(service_fee);
        self.profit_address().set(profit_address);
        self.claim_fee().set(claim_fee);
        self.limit().set(limit);
    }
}
