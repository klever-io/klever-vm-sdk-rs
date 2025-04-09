#![no_std]

use klever_sc::imports::*;

#[klever_sc::contract]
pub trait Deployer {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // storage map deployed contracts
    #[view(getDeployedContracts)]
    #[storage_mapper("deployed_contracts")]
    fn deployed_contracts(&self) -> VecMapper<ManagedAddress>;

    #[endpoint(deploy)]
    fn deploy(&self, address_source: &ManagedAddress) -> ManagedAddress {
        let gas_left = self.blockchain().get_gas_left();
        let len = self.deployed_contracts().len();
        // load args
        let mut arguments = ManagedArgBuffer::new();
        arguments.push_arg(BigUint::from(len));

        let child_contract_address = self
            .tx()
            .gas(gas_left)
            .raw_deploy()
            .arguments_raw(arguments)
            .from_source(address_source.clone())
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewManagedAddress)
            .sync_call();

        self.deployed_contracts().push(&child_contract_address);

        child_contract_address
    }

    #[endpoint(deleteLast)]
    fn delete_last(&self) -> bool {
        let len = self.deployed_contracts().len();
        if len == 0 {
            return false;
        }
        let contract_address = self.deployed_contracts().get(len);
        let gas_left = self.blockchain().get_gas_left();
        let args = &ManagedArgBuffer::new();
        self.send_raw()
            .delete_contract(&contract_address, gas_left, args);
        self.deployed_contracts().swap_remove(len);
        true
    }

    #[endpoint(upgradeDeployed)]
    fn upgrade_deployed(&self, index: usize, address_source: &ManagedAddress) {
        let len = self.deployed_contracts().len();
        require!(index <= len, "Index out of bounds");
        let deployed_contract = self.deployed_contracts().get(index);

        self.tx()
            .gas(self.blockchain().get_gas_left())
            .to(deployed_contract.clone())
            .raw_upgrade()
            .arguments_raw(ManagedArgBuffer::new())
            .from_source(address_source.clone())
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .upgrade_sync_call();
    }
}
