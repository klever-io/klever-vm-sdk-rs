use klever_sc::imports::*;
use klever_sc::derive_imports::*;

use crate::storage;

#[klever_sc::module]
pub trait StorageSetters: storage::Storage {
    #[only_owner]
    #[endpoint]
    fn set_profit_address(&self, address: ManagedAddress) {
        self.profit_address().set(address);
    }

    #[only_owner]
    #[endpoint]
    fn set_fee(&self, fee: u32) {
        self.service_fee().set(fee);
    }

    #[only_owner]
    #[endpoint]
    fn set_limit(&self, limit: u32) {
        self.limit().set(limit);
    }

    #[only_owner]
    #[endpoint]
    fn set_claim_fee(&self, fee: BigUint) {
        self.claim_fee().set(fee);
    }
}