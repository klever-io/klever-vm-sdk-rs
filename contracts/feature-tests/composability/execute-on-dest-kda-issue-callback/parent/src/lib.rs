#![no_std]

use klever_sc::imports::*;

// Base cost for standalone + estimate cost of actual sc call
const ISSUE_EXPECTED_GAS_COST: u64 = 90_000_000 + 25_000_000;

#[klever_sc::contract]
pub trait Parent {
    #[proxy]
    fn child_proxy(&self, to: ManagedAddress) -> child::Proxy<Self::Api>;

    #[init]
    fn init(&self) {}

    #[payable("KLV")]
    #[endpoint]
    fn deposit(&self) {}

    #[endpoint(deployChildContract)]
    fn deploy_child_contract(&self, code: ManagedBuffer) {
        let (child_contract_address, _) = self.send_raw().deploy_contract(
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            &code,
            CodeMetadata::DEFAULT,
            &ManagedArgBuffer::new(),
        );

        self.child_contract_address().set(&child_contract_address);
    }

    #[payable("KLV")]
    #[endpoint(executeOnDestIssueToken)]
    fn execute_on_dest_issue_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let issue_cost = self.call_value().klv_value();
        let child_contract_adress = self.child_contract_address().get();
        let _: IgnoreValue = self
            .child_proxy(child_contract_adress)
            .issue_wrapped_klv(token_display_name, token_ticker, initial_supply)
            .with_klv_transfer(issue_cost.clone_value())
            .gas(ISSUE_EXPECTED_GAS_COST)
            .execute_on_dest_context();
    }

    // storage

    #[view(getChildContractAddress)]
    #[storage_mapper("childContractAddress")]
    fn child_contract_address(&self) -> SingleValueMapper<ManagedAddress>;
}
