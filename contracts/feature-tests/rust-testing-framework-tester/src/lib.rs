#![no_std]

use klever_sc::derive_imports::*;
use klever_sc::imports::*;

pub mod dummy_module;

#[derive(TopEncode, TopDecode, TypeAbi, Clone, Debug, PartialEq, Eq)]
pub struct NftDummyAttributes {
    pub creation_epoch: u64,
    pub cool_factor: u8,
}

pub struct StructWithManagedTypes<M: ManagedTypeApi> {
    pub big_uint: BigUint<M>,
    pub buffer: ManagedBuffer<M>,
}

#[klever_sc::contract]
pub trait RustTestingFrameworkTester: dummy_module::DummyModule {
    #[init]
    fn init(&self) -> ManagedBuffer {
        self.total_value().set(BigUint::from(1u32));
        b"constructor-result".into()
    }

    #[endpoint]
    fn sum(&self, first: BigUint, second: BigUint) -> BigUint {
        first + second
    }

    #[endpoint]
    fn sum_sc_result(&self, first: BigUint, second: BigUint) -> BigUint {
        require!(first > 0 && second > 0, "Non-zero required");
        first + second
    }

    #[endpoint]
    fn get_caller_legacy(&self) -> Address {
        #[allow(deprecated)]
        self.blockchain().get_caller_legacy()
    }

    #[endpoint]
    fn get_klv_balance(&self) -> BigUint {
        self.blockchain().get_sc_balance(&TokenIdentifier::klv(), 0)
    }

    #[endpoint]
    fn get_kda_balance(&self, token_id: TokenIdentifier, nonce: u64) -> BigUint {
        self.blockchain().get_sc_balance(&token_id, nonce)
    }

    #[payable("KLV")]
    #[endpoint]
    fn receive_klv(&self) -> BigUint {
        self.call_value().klv_value().clone_value()
    }

    #[payable("KLV")]
    #[endpoint]
    fn recieve_klv_half(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = &*self.call_value().klv_value() / 2u32;
        self.send()
            .direct_kda(&caller, &TokenIdentifier::klv(), 0, &payment_amount);
    }

    #[payable("*")]
    #[endpoint]
    fn receive_kda(&self) -> (TokenIdentifier, BigUint) {
        let payment = self.call_value().single_kda();
        (payment.token_identifier, payment.amount)
    }

    #[payable("*")]
    #[endpoint]
    fn reject_payment(&self) {
        sc_panic!("No payment allowed!");
    }

    #[payable("*")]
    #[endpoint]
    fn receive_kda_half(&self) {
        let payment = self.call_value().single_kda();
        let amount = payment.amount / 2u32;

        self.tx()
            .to(ToCaller)
            .single_kda(&payment.token_identifier, 0, &amount)
            .transfer();
    }

    #[payable("*")]
    #[endpoint]
    fn receive_multi_kda(&self) -> ManagedVec<KdaTokenPayment<Self::Api>> {
        self.call_value().all_kda_transfers().clone_value()
    }

    #[payable("*")]
    #[endpoint]
    fn send_nft(
        &self,
        to: ManagedAddress,
        token_id: TokenIdentifier,
        nft_nonce: u64,
        amount: BigUint,
    ) {
        self.tx()
            .to(&to)
            .single_kda(&token_id, nft_nonce, &amount)
            .transfer();
    }

    #[endpoint]
    fn mint_kda(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().kda_mint(&token_id, nonce, &amount);
    }

    #[endpoint]
    fn burn_kda(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().kda_burn(&token_id, nonce, &amount);
    }

    #[endpoint]
    fn get_block_epoch(&self) -> u64 {
        self.blockchain().get_block_epoch()
    }

    #[endpoint]
    fn get_block_nonce(&self) -> u64 {
        self.blockchain().get_block_nonce()
    }

    #[endpoint]
    fn get_block_timestamp(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    #[endpoint]
    fn get_random_buffer_once(&self, len: usize) -> ManagedBuffer {
        ManagedBuffer::new_random(len)
    }

    #[endpoint]
    fn get_random_buffer_twice(&self, len1: usize, len2: usize) -> (ManagedBuffer, ManagedBuffer) {
        (
            ManagedBuffer::new_random(len1),
            ManagedBuffer::new_random(len2),
        )
    }

    #[endpoint]
    fn call_other_contract_execute_on_dest(&self, other_sc_address: ManagedAddress) -> BigUint {
        let gas_left = self.blockchain().get_gas_left();
        let call_result = self
            .tx()
            .to(&other_sc_address)
            .gas(gas_left)
            .raw_call("getTotalValue")
            .returns(ReturnsRawResult)
            .sync_call();
        if let Some(raw_value) = call_result.try_get(0) {
            BigUint::from_bytes_be_buffer(&raw_value)
        } else {
            BigUint::zero()
        }
    }

    #[endpoint]
    fn call_other_contract_add_async_call(&self, other_sc_address: ManagedAddress, value: BigUint) {
        self.tx()
            .to(&other_sc_address)
            .raw_call("add")
            .argument(&value)
            .sync_call();
    }

    #[endpoint(getTotalValue)]
    fn get_total_value(&self) -> BigUint {
        self.total_value().get()
    }

    #[endpoint]
    fn execute_on_dest_add_value(&self, other_sc_address: ManagedAddress, value: BigUint) {
        let gas_left = self.blockchain().get_gas_left();
        self.tx()
            .to(&other_sc_address)
            .gas(gas_left)
            .raw_call("addValue")
            .argument(&value)
            .sync_call();
    }

    #[endpoint(addValue)]
    fn add(&self, value: BigUint) {
        let caller = self.blockchain().get_caller();

        self.total_value().update(|val| *val += &value);
        self.value_per_caller(&caller).update(|val| *val += value);
    }

    #[endpoint]
    fn panic(&self) {
        sc_panic!("Oh no!");
    }

    fn get_val(&self) -> BigUint {
        self.total_value().get()
    }

    #[storage_mapper("totalValue")]
    fn total_value(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("valuePerCaller")]
    fn value_per_caller(&self, caller: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("callbackExecuted")]
    fn callback_executed(&self) -> SingleValueMapper<bool>;
}
