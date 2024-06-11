#![no_std]

use klever_sc::imports::*;

mod data_struct;

use data_struct::DataStruct;

#[klever_sc::contract]
pub trait SaveData {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn save_data(&self, data: DataStruct<Self::Api>) {
        let address = &self.blockchain().get_caller();

        self.data_info(address).set(data);
    }

    #[view(getData)]
    #[storage_mapper("dataInfo")]
    fn data_info(
        &self,
        address: &ManagedAddress<Self::Api>,
    ) -> SingleValueMapper<DataStruct<Self::Api>>;
}
