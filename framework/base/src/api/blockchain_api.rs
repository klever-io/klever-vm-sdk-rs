use super::{HandleTypeInfo, ManagedTypeApi, ManagedTypeApiImpl, RawHandle};
use crate::types::heap::{Address, Box, H256};

pub trait BlockchainApi: ManagedTypeApi {
    type BlockchainApiImpl: BlockchainApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl;
}

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait BlockchainApiImpl: ManagedTypeApiImpl {
    fn get_caller_legacy(&self) -> Address;

    fn load_caller_managed(&self, dest: Self::ManagedBufferHandle) {
        self.mb_overwrite(dest, self.get_caller_legacy().as_bytes());
    }

    fn get_sc_address_legacy(&self) -> Address;

    fn load_sc_address_managed(&self, dest: Self::ManagedBufferHandle) {
        self.mb_overwrite(dest, self.get_sc_address_legacy().as_bytes())
    }

    fn load_owner_address_managed(&self, dest: Self::ManagedBufferHandle);

    fn is_smart_contract_legacy(&self, address: &Address) -> bool;

    fn is_smart_contract(&self, address_handle: Self::ManagedBufferHandle) -> bool {
        let mut address = Address::zero();
        let _ = self.mb_load_slice(address_handle, 0, address.as_mut());
        self.is_smart_contract_legacy(&address)
    }

    fn load_balance_legacy(&self, dest: Self::BigIntHandle, address: &Address);

    fn load_balance(&self, dest: Self::BigIntHandle, address_handle: Self::ManagedBufferHandle) {
        let mut address = Address::zero();
        let _ = self.mb_load_slice(address_handle, 0, address.as_mut());
        self.load_balance_legacy(dest, &address);
    }

    fn load_state_root_hash_managed(&self, dest: Self::ManagedBufferHandle);

    fn get_tx_hash_legacy(&self) -> H256;

    fn load_tx_hash_managed(&self, dest: Self::ManagedBufferHandle) {
        self.mb_overwrite(dest, self.get_tx_hash_legacy().as_bytes());
    }

    fn get_gas_left(&self) -> u64;

    fn get_block_timestamp(&self) -> u64;

    fn get_block_nonce(&self) -> u64;

    fn get_block_round(&self) -> u64;

    fn get_block_epoch(&self) -> u64;

    fn load_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle);

    fn get_prev_block_timestamp(&self) -> u64;

    fn get_prev_block_nonce(&self) -> u64;

    fn get_prev_block_round(&self) -> u64;

    fn get_prev_block_epoch(&self) -> u64;

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]>;

    fn load_prev_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle) {
        self.mb_overwrite(dest, self.get_prev_block_random_seed_legacy().as_slice());
    }

    fn load_kda_balance(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        nonce: u64,
        dest: Self::BigIntHandle,
    );

    #[allow(clippy::too_many_arguments)]
    fn managed_get_user_kda(
        &self,
        address_handle: RawHandle,
        ticker_handle: RawHandle,
        nonce: u64,
        balance_handle: RawHandle,
        frozen_handle: RawHandle,
        last_claim_handle: RawHandle,
        buckets_handle: RawHandle,
        mime_handle: RawHandle,
        metadata_handle: RawHandle,
    );

    #[allow(clippy::too_many_arguments)]
    fn managed_get_kda_token_data(
        &self,
        address_handle: RawHandle,
        ticker_handle: RawHandle,
        nonce: u64,
        precision_handle: RawHandle,
        id_handle: RawHandle,
        name_handle: RawHandle,
        creator_handle: RawHandle,
        logo_handle: RawHandle,
        uris_handle: RawHandle,
        initial_supply_handle: RawHandle,
        circulating_supply_handle: RawHandle,
        max_supply_handle: RawHandle,
        minted_handle: RawHandle,
        burned_handle: RawHandle,
        royalties_handle: RawHandle,
        properties_handle: RawHandle,
        attributes_handle: RawHandle,
        roles_handle: RawHandle,
        issue_date_handle: RawHandle,
    );

    fn managed_get_kda_roles(
        &self,
        ticker_handle: RawHandle,
        roles_handle: RawHandle,
    );

    fn managed_get_back_transfers(
        &self,
        kda_transfer_value_handle: RawHandle,
        call_value_handle: RawHandle,
    );

}
