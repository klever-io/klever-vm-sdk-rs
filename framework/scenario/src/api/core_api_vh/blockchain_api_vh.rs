use klever_sc::{
    api::{BlockchainApi, BlockchainApiImpl, HandleConstraints, ManagedBufferApiImpl, RawHandle},
    types::{Address, H256},
};

use crate::api::{i32_to_bool, VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> BlockchainApi for VMHooksApi<VHB> {
    type BlockchainApiImpl = Self;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> BlockchainApiImpl for VMHooksApi<VHB> {
    fn get_caller_legacy(&self) -> Address {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn load_caller_managed(&self, dest: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.with_vm_hooks(|vh| vh.managed_caller(dest.get_raw_handle_unchecked()));
    }

    fn get_sc_address_legacy(&self) -> Address {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn load_sc_address_managed(&self, dest: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.with_vm_hooks(|vh| vh.managed_sc_address(dest.get_raw_handle_unchecked()));
    }

    fn load_owner_address_managed(&self, dest: Self::ManagedBufferHandle) {
        self.with_vm_hooks(|vh| vh.managed_owner_address(dest.get_raw_handle_unchecked()));
    }

    fn is_smart_contract_legacy(&self, _address: &Address) -> bool {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn is_smart_contract(&self, address_handle: Self::ManagedBufferHandle) -> bool {
        self.assert_live_handle(&address_handle);
        let result = self.with_temp_address_ptr(address_handle, |address_ptr| {
            self.with_vm_hooks(|vh| vh.is_smart_contract(address_ptr))
        });
        i32_to_bool(result)
    }

    fn load_balance_legacy(&self, _dest: Self::BigIntHandle, _address: &Address) {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn load_balance(&self, dest: Self::BigIntHandle, address_handle: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.assert_live_handle(&address_handle);
        self.with_temp_address_ptr(address_handle, |address_ptr: isize| {
            self.with_vm_hooks(|vh| {
                vh.big_int_get_external_balance(address_ptr, dest.get_raw_handle_unchecked())
            })
        });
    }

    fn load_state_root_hash_managed(&self, _dest: Self::ManagedBufferHandle) {
        panic!("state root hash not implemented")
    }

    fn get_tx_hash_legacy(&self) -> H256 {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn load_tx_hash_managed(&self, dest: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.with_vm_hooks(|vh| vh.managed_get_original_tx_hash(dest.get_raw_handle_unchecked()));
    }

    fn get_gas_left(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_gas_left()) as u64
    }

    fn get_block_timestamp(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_block_timestamp()) as u64
    }

    fn get_block_nonce(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_block_nonce()) as u64
    }

    fn get_block_round(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_block_round()) as u64
    }

    fn get_block_epoch(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_block_epoch()) as u64
    }

    fn load_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.with_vm_hooks(|vh| vh.managed_get_block_random_seed(dest.get_raw_handle_unchecked()));
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_prev_block_timestamp()) as u64
    }

    fn get_prev_block_nonce(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_prev_block_nonce()) as u64
    }

    fn get_prev_block_round(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_prev_block_round()) as u64
    }

    fn get_prev_block_epoch(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_prev_block_epoch()) as u64
    }

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn load_prev_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.with_vm_hooks(|vh| {
            vh.managed_get_prev_block_random_seed(dest.get_raw_handle_unchecked())
        });
    }

    fn load_kda_balance(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        nonce: u64,
        dest: Self::BigIntHandle,
    ) {
        self.assert_live_handle(&address_handle);
        self.assert_live_handle(&token_id_handle);
        let token_id_len = self.mb_len(token_id_handle.clone());
        self.with_temp_address_ptr(address_handle, |address_ptr| {
            self.with_temp_buffer_ptr(token_id_handle, token_id_len, |token_id_ptr| {
                self.with_vm_hooks(|vh| {
                    vh.big_int_get_kda_external_balance(
                        address_ptr,
                        token_id_ptr,
                        token_id_len as isize,
                        nonce as i64,
                        dest.get_raw_handle_unchecked(),
                    );
                })
            })
        });
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
        self.with_vm_hooks(|vh| {
            vh.managed_get_user_kda(
                address_handle,
                ticker_handle,
                nonce as i64,
                balance_handle,
                frozen_handle,
                last_claim_handle,
                buckets_handle,
                mime_handle,
                metadata_handle,
            );
        });
    }

    fn managed_get_sft_metadata(
        &self,
        ticker_handle: RawHandle,
        nonce: u64,
        data_handle: RawHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.managed_get_sft_metadata(ticker_handle, nonce as i64, data_handle)
        });
    }

    fn managed_acc_has_perm(
        &self,
        ops: i64,
        source_acc_addr: RawHandle,
        target_acc_addr: RawHandle,
    ) -> bool {
        self.with_vm_hooks(|vh| {
            vh.managed_acc_has_perm(
                ops,
                source_acc_addr,
                target_acc_addr,
            )
        })
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
        self.with_vm_hooks(|vh| {
            vh.managed_get_kda_token_data(
                address_handle,
                ticker_handle,
                nonce as i64,
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
            )
        });
    }

    fn managed_get_kda_roles(&self, ticker_handle: RawHandle, roles_handle: RawHandle) {
        self.with_vm_hooks(|vh| vh.managed_get_kda_roles(ticker_handle, roles_handle));
    }

    fn managed_get_back_transfers(
        &self,
        kda_transfer_value_handle: RawHandle,
        call_value_handle: RawHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.managed_get_back_transfers(kda_transfer_value_handle, call_value_handle)
        });
    }

    fn managed_get_code_metadata(
        &self,
        address_handle: Self::ManagedBufferHandle,
        response_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.managed_get_code_metadata(
                address_handle.get_raw_handle_unchecked(),
                response_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn managed_is_builtin_function(&self, function_name_handle: Self::ManagedBufferHandle) -> bool {
        i32_to_bool(self.with_vm_hooks(|vh| {
            vh.managed_is_builtin_function(function_name_handle.get_raw_handle_unchecked())
        }))
    }
}
