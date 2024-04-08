use alloc::boxed::Box;

use crate::{
    api::{BlockchainApi, BlockchainApiImpl, RawHandle},
    types::heap::{Address, H256},
};

use super::UncallableApi;

impl BlockchainApi for UncallableApi {
    type BlockchainApiImpl = UncallableApi;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl {
        unreachable!()
    }
}

impl BlockchainApiImpl for UncallableApi {
    fn get_sc_address_legacy(&self) -> Address {
        unreachable!()
    }

    fn load_owner_address_managed(&self, _dest: Self::ManagedBufferHandle) {
        unreachable!()
    }

    fn is_smart_contract_legacy(&self, _address: &Address) -> bool {
        unreachable!()
    }

    fn get_caller_legacy(&self) -> Address {
        unreachable!()
    }

    fn load_balance_legacy(&self, _dest: Self::BigIntHandle, _address: &Address) {
        unreachable!()
    }

    fn load_state_root_hash_managed(&self, _dest: Self::ManagedBufferHandle) {
        unreachable!()
    }

    fn get_tx_hash_legacy(&self) -> H256 {
        unreachable!()
    }

    fn get_gas_left(&self) -> u64 {
        unreachable!()
    }

    fn get_block_timestamp(&self) -> u64 {
        unreachable!()
    }

    fn get_block_nonce(&self) -> u64 {
        unreachable!()
    }

    fn get_block_round(&self) -> u64 {
        unreachable!()
    }

    fn get_block_epoch(&self) -> u64 {
        unreachable!()
    }

    fn load_block_random_seed_managed(&self, _dest: Self::ManagedBufferHandle) {
        unreachable!()
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_nonce(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_round(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_epoch(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        unreachable!()
    }

    fn load_kda_balance(
        &self,
        _address_handle: Self::ManagedBufferHandle,
        _token_id_handle: Self::ManagedBufferHandle,
        _nonce: u64,
        _dest: Self::BigIntHandle,
    ) {
        unreachable!()
    }

    fn managed_get_user_kda(
        &self,
        _address_handle: RawHandle,
        _ticker_handle: RawHandle,
        _nonce: u64,
        _balance_handle: RawHandle,
        _frozen_handle: RawHandle,
        _last_claim_handle: RawHandle,
        _buckets_handle: RawHandle,
        _mime_handle: RawHandle,
        _metadata_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn managed_get_kda_token_data(
        &self,
        _address_handle: RawHandle,
        _ticker_handle: RawHandle,
        _nonce: u64,
        _precision: RawHandle,
        _id_handle: RawHandle,
        _name_handle: RawHandle,
        _creator_handle: RawHandle,
        _logo_handle: RawHandle,
        _uris_handle: RawHandle,
        _initial_supply_handle: RawHandle,
        _circulating_supply_handle: RawHandle,
        _max_supply_handle: RawHandle,
        _minted_handle: RawHandle,
        _burned_handle: RawHandle,
        _royalties_handle: RawHandle,
        _properties_handle: RawHandle,
        _attributes_handle: RawHandle,
        _roles_handle: RawHandle,
        _issue_date_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn managed_get_sft_metadata(
        &self,
        _ticker_handle: RawHandle,
        _nonce: u64,
        _data_handle: RawHandle,
    ){
        unreachable!()
    }

    fn managed_get_kda_roles(
        &self,
        _ticker_handle: RawHandle,
        _roles_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn managed_get_back_transfers(
        &self,
        _kda_transfer_value_handle: RawHandle,
        _call_value_handle: RawHandle,
    ) {
        unreachable!()
    }
}
