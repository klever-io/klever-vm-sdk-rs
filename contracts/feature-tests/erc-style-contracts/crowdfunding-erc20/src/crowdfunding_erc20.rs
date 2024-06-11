#![no_std]

use klever_sc::imports::*;
use klever_sc::derive_imports::*;

#[derive(TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone, Copy)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

#[klever_sc::contract]
pub trait Crowdfunding {
    #[init]
    fn init(&self, target: BigUint, deadline: u64, erc20_contract_address: ManagedAddress) {
        self.erc20_contract_address().set(&erc20_contract_address);
        self.target().set(&target);
        self.deadline().set(deadline);
    }

    #[endpoint]
    fn fund(&self, token_amount: BigUint) {
        require!(
            self.blockchain().get_block_nonce() <= self.deadline().get(),
            "cannot fund after deadline"
        );

        let caller = self.blockchain().get_caller();
        let erc20_address = self.erc20_contract_address().get();
        let cf_contract_address = self.blockchain().get_sc_address();

        self.erc20_proxy(erc20_address)
            .transfer_from(caller.clone(), cf_contract_address, token_amount.clone())
            .execute_on_dest_context::<IgnoreValue>();

        self.deposit(&caller.clone())
            .update(|deposit| *deposit += &token_amount.clone());
        self.total_balance()
            .update(|balance| *balance += token_amount.clone());
    }

    #[view]
    fn status(&self) -> Status {
        if self.blockchain().get_block_nonce() <= self.deadline().get() {
            Status::FundingPeriod
        } else if self.blockchain().get_sc_balance(&TokenIdentifier::klv(), 0)
            >= self.target().get()
        {
            Status::Successful
        } else {
            Status::Failed
        }
    }

    #[endpoint]
    fn claim(&self) {
        match self.status() {
            Status::FundingPeriod => sc_panic!("cannot claim before deadline"),
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                if caller != self.blockchain().get_owner_address() {
                    sc_panic!("only owner can claim successful funding");
                }

                let balance = self.total_balance().get();
                self.total_balance().clear();

                let erc20_address = self.erc20_contract_address().get();

                self.erc20_proxy(erc20_address)
                    .transfer(caller, balance)
                    .execute_on_dest_context::<IgnoreValue>();
            },
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get();

                if deposit > 0 {
                    self.deposit(&caller).clear();

                    let erc20_address = self.erc20_contract_address().get();

                    self.erc20_proxy(erc20_address)
                        .transfer(caller, deposit)
                        .execute_on_dest_context::<IgnoreValue>();
                }
            },
        }
    }

    // proxy
    #[proxy]
    fn erc20_proxy(&self, to: ManagedAddress) -> erc20::Proxy<Self::Api>;

    // storage

    #[view(get_target)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[view(get_deadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    #[view(get_deposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(get_erc20_contract_address)]
    #[storage_mapper("erc20ContractAddress")]
    fn erc20_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(get_total_balance)]
    #[storage_mapper("erc20Balance")]
    fn total_balance(&self) -> SingleValueMapper<BigUint>;
}
