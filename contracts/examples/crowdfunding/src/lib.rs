#![no_std]

mod crowdfunding_data;
pub mod crowdfunding_helpers;
mod crowdfunding_methods;
mod crowdfunding_views;
mod storage;
mod storage_setters;

#[klever_sc::contract]
pub trait Crowdfunding:
    crowdfunding_methods::CrowdfundingMethods
    + storage_setters::StorageSetters
    + crowdfunding_views::CrowdfundingViews
    + storage::Storage
{
    #[init]
    fn init(
        &self,
        service_fee: u32,
        limit: u32,
        profit_address: ManagedAddress,
        claim_fee: BigUint,
    ) {
        self.service_fee().set(service_fee);
        self.profit_address().set(profit_address);
        self.claim_fee().set(claim_fee);
        self.limit().set(limit);
    }
}
