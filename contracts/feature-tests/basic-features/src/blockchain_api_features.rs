klever_sc::imports!();

/// Contains all events that can be emitted by the contract.
#[klever_sc::module]
pub trait BlockchainApiFeatures {
    #[endpoint]
    fn get_caller(&self) -> ManagedAddress {
        self.blockchain().get_caller()
    }

    #[endpoint]
    fn get_owner_address(&self) -> ManagedAddress {
        self.blockchain().get_owner_address()
    }

    #[endpoint]
    fn is_smart_contract(&self, address: &ManagedAddress) -> bool {
        self.blockchain().is_smart_contract(address)
    }

    #[endpoint]
    fn get_state_root_hash(&self) -> ManagedByteArray<Self::Api, 32> {
        self.blockchain().get_state_root_hash()
    }

    #[endpoint]
    fn get_tx_hash(&self) -> ManagedByteArray<Self::Api, 32> {
        self.blockchain().get_tx_hash()
    }

    #[endpoint]
    fn get_gas_left(&self) -> u64 {
        self.blockchain().get_gas_left()
    }
}
