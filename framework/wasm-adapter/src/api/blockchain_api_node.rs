use crate::api::{
    managed_types::managed_buffer_api_node::{
        unsafe_buffer_load_address, unsafe_buffer_load_token_identifier,
    },
    VmApiImpl,
};
use klever_sc::{
    api::{BlockchainApi, BlockchainApiImpl, ManagedBufferApiImpl, RawHandle},
    types::heap::{Address, Box, H256},
};

extern "C" {
    // address utils
    fn getSCAddress(resultOffset: *mut u8);

    fn managedSCAddress(resultHandle: i32);
    fn managedOwnerAddress(resultHandle: i32);

    fn getCaller(resultOffset: *mut u8);

    fn managedCaller(resultHandle: i32);

    fn isSmartContract(address_ptr: *const u8) -> i32;

    /// Currently not used.
    #[allow(dead_code)]
    fn getFunction(functionOffset: *const u8) -> i32;

    fn getGasLeft() -> i64;
    fn getBlockTimestamp() -> i64;
    fn getBlockNonce() -> i64;
    fn getBlockRound() -> i64;
    fn getBlockEpoch() -> i64;
    fn getPrevBlockTimestamp() -> i64;
    fn getPrevBlockNonce() -> i64;
    fn getPrevBlockRound() -> i64;
    fn getPrevBlockEpoch() -> i64;
    fn getPrevBlockRandomSeed(resultOffset: *const u8);
    fn getOriginalTxHash(resultOffset: *const u8);

    // Managed versions of the above
    fn managedGetPrevBlockRandomSeed(resultHandle: i32);
    fn managedGetBlockRandomSeed(resultHandle: i32);
    fn managedGetStateRootHash(resultHandle: i32);
    fn managedGetOriginalTxHash(resultHandle: i32);

    // big int API
    fn bigIntGetExternalBalance(address_ptr: *const u8, dest: i32);
    fn bigIntGetKDAExternalBalance(
        address_ptr: *const u8,
        tokenIDOffset: *const u8,
        tokenIDLen: i32,
        nonce: i64,
        dest: i32,
    );

    fn managedGetUserKDA(
        address_handle: i32,
        ticker_handle: i32,
        nonce: u64,
        balance_handle: i32,
        frozen_handle: i32,
        last_claim_handle: i32,
        buckets_handle: i32,
        mime_handle: i32,
        metadata_handle: i32,
    );

    fn managedGetSftMetadata(ticker_handle: i32, nonce: u64, data_handle: i32);

    fn managedGetKDATokenData(
        address_handle: i32,
        ticker_handle: i32,
        nonce: u64,
        precision_handle: i32,
        id_handle: i32,
        name_handle: i32,
        creator_handle: i32,
        admin_handle: i32,
        logo_handle: i32,
        uris_handle: i32,
        initial_supply_handle: i32,
        circulating_supply_handle: i32,
        max_supply_handle: i32,
        minted_handle: i32,
        burned_handle: i32,
        royalties_handle: i32,
        properties_handle: i32,
        attributes_handle: i32,
        roles_handle: i32,
        issue_date_handle: i32,
    );

    fn managedGetKDARoles(ticker_handle: i32, roles_handle: i32);

    fn managedGetBackTransfers(kdaTransfersValueHandle: i32, callValueHandle: i32);
}

impl BlockchainApi for VmApiImpl {
    type BlockchainApiImpl = VmApiImpl;

    #[inline]
    fn blockchain_api_impl() -> Self::BlockchainApiImpl {
        VmApiImpl {}
    }
}

impl BlockchainApiImpl for VmApiImpl {
    #[inline]
    fn get_caller_legacy(&self) -> Address {
        unsafe {
            let mut res = Address::zero();
            getCaller(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    fn load_caller_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedCaller(dest);
        }
    }

    #[inline]
    fn get_sc_address_legacy(&self) -> Address {
        unsafe {
            let mut res = Address::zero();
            getSCAddress(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    fn load_sc_address_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedSCAddress(dest);
        }
    }

    #[inline]
    fn load_owner_address_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedOwnerAddress(dest);
        }
    }
    #[inline]
    fn is_smart_contract_legacy(&self, address: &Address) -> bool {
        unsafe { isSmartContract(address.as_ref().as_ptr()) > 0 }
    }

    #[inline]
    fn is_smart_contract(&self, address_handle: Self::ManagedBufferHandle) -> bool {
        unsafe { isSmartContract(unsafe_buffer_load_address(address_handle)) > 0 }
    }

    #[inline]
    fn load_balance_legacy(&self, dest: Self::BigIntHandle, address: &Address) {
        unsafe {
            bigIntGetExternalBalance(address.as_ref().as_ptr(), dest);
        }
    }

    #[inline]
    fn load_balance(&self, dest: Self::BigIntHandle, address_handle: Self::ManagedBufferHandle) {
        unsafe {
            bigIntGetExternalBalance(unsafe_buffer_load_address(address_handle), dest);
        }
    }

    #[inline]
    fn load_state_root_hash_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedGetStateRootHash(dest);
        }
    }

    #[inline]
    fn get_tx_hash_legacy(&self) -> H256 {
        unsafe {
            let mut res = H256::zero();
            getOriginalTxHash(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    fn load_tx_hash_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedGetOriginalTxHash(dest);
        }
    }

    #[inline]
    fn get_gas_left(&self) -> u64 {
        unsafe { getGasLeft() as u64 }
    }

    #[inline]
    fn get_block_timestamp(&self) -> u64 {
        unsafe { getBlockTimestamp() as u64 }
    }

    #[inline]
    fn get_block_nonce(&self) -> u64 {
        unsafe { getBlockNonce() as u64 }
    }

    #[inline]
    fn get_block_round(&self) -> u64 {
        unsafe { getBlockRound() as u64 }
    }

    #[inline]
    fn get_block_epoch(&self) -> u64 {
        unsafe { getBlockEpoch() as u64 }
    }

    #[inline]
    fn load_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedGetBlockRandomSeed(dest);
        }
    }

    #[inline]
    fn get_prev_block_timestamp(&self) -> u64 {
        unsafe { getPrevBlockTimestamp() as u64 }
    }

    #[inline]
    fn get_prev_block_nonce(&self) -> u64 {
        unsafe { getPrevBlockNonce() as u64 }
    }

    #[inline]
    fn get_prev_block_round(&self) -> u64 {
        unsafe { getPrevBlockRound() as u64 }
    }

    #[inline]
    fn get_prev_block_epoch(&self) -> u64 {
        unsafe { getPrevBlockEpoch() as u64 }
    }

    #[inline]
    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        unsafe {
            let mut res = [0u8; 48];
            getPrevBlockRandomSeed(res.as_mut_ptr());
            Box::new(res)
        }
    }

    #[inline]
    fn load_prev_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedGetPrevBlockRandomSeed(dest);
        }
    }

    fn load_kda_balance(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        nonce: u64,
        dest: Self::BigIntHandle,
    ) {
        let token_identifier_len = self.mb_len(token_id_handle);
        unsafe {
            bigIntGetKDAExternalBalance(
                unsafe_buffer_load_address(address_handle),
                unsafe_buffer_load_token_identifier(token_id_handle),
                token_identifier_len as i32,
                nonce as i64,
                dest,
            );
        }
    }

    fn managed_get_sft_metadata(
        &self,
        token_handle: RawHandle,
        nonce: u64,
        data_handle: RawHandle,
    ) {
        unsafe {
            managedGetSftMetadata(token_handle, nonce, data_handle);
        }
    }

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
    ) {
        unsafe {
            managedGetUserKDA(
                address_handle,
                ticker_handle,
                nonce,
                balance_handle,
                frozen_handle,
                last_claim_handle,
                buckets_handle,
                mime_handle,
                metadata_handle,
            );
        }
    }

    fn managed_get_kda_token_data(
        &self,
        address_handle: RawHandle,
        ticker_handle: RawHandle,
        nonce: u64,
        precision_handle: RawHandle,
        id_handle: RawHandle,
        name_handle: RawHandle,
        creator_handle: RawHandle,
        admin_handle: RawHandle,
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
    ) {
        unsafe {
            managedGetKDATokenData(
                address_handle,
                ticker_handle,
                nonce,
                precision_handle,
                id_handle,
                name_handle,
                creator_handle,
                admin_handle,
                logo_handle,
                uris_handle,
                initial_supply_handle,
                circulating_supply_handle,
                max_supply_handle,
                minted_handle,
                burned_handle,
                royalties_handle,
                properties_handle,
                attributes_handle,
                roles_handle,
                issue_date_handle,
            );
        }
    }

    fn managed_get_kda_roles(&self, ticker_handle: RawHandle, roles_handle: RawHandle) {
        unsafe {
            managedGetKDARoles(ticker_handle, roles_handle);
        }
    }

    fn managed_get_back_transfers(
        &self,
        kda_transfer_value_handle: RawHandle,
        call_value_handle: RawHandle,
    ) {
        unsafe {
            managedGetBackTransfers(kda_transfer_value_handle, call_value_handle);
        }
    }
}
