use num_bigint::BigInt;

use crate::{
    tx_execution::vm_builtin_function_names::*,
    types::{RawHandle, VMAddress},
    vm_hooks::VMHooksHandlerSource,
    world_mock::{AccountData, KdaData, KdaInstance},
};
// use num_bigint::BigInt;
// use num_traits::Zero;

// The Go VM doesn't do it, but if we change that, we can enable it easily here too via this constant.
const KDA_TOKEN_DATA_FUNC_RESETS_VALUES: bool = false;

const VM_BUILTIN_FUNCTION_NAMES: [&str; 22] = [
    KLEVER_TRANSFER_FUNC_NAME,
    KLEVER_CREATE_ASSET_FUNC_NAME,
    KLEVER_FREEZE_FUNC_NAME,
    KLEVER_UNFREEZE_FUNC_NAME,
    KLEVER_DELEGATE_FUNC_NAME,
    KLEVER_UNDELEGATE_FUNC_NAME,
    KLEVER_WITHDRAW_FUNC_NAME,
    KLEVER_CLAIM_FUNC_NAME,
    KLEVER_ASSET_TRIGGER_FUNC_NAME,
    KLEVER_SET_ACCOUNT_NAME_FUNC_NAME,
    KLEVER_VOTE_FUNC_NAME,
    KLEVER_CONFIG_ITO_FUNC_NAME,
    KLEVER_BUY_FUNC_NAME,
    KLEVER_SELL_FUNC_NAME,
    KLEVER_CANCEL_MARKET_ORDER_FUNC_NAME,
    KLEVER_CREATE_MARKETPLACE_FUNC_NAME,
    KLEVER_CONFIG_MARKETPLACE_FUNC_NAME,
    KLEVER_UPDATE_ACCOUNT_PERMISSION,
    KLEVER_DEPOSIT_FUNC_NAME,
    KLEVER_ITO_TRIGGER_FUNC_NAME,
    CHANGE_OWNER_BUILTIN_FUNC_NAME,
    UPGRADE_CONTRACT_FUNC_NAME,
];

pub trait VMHooksBlockchain: VMHooksHandlerSource {
    fn is_contract_address(&self, address_bytes: &[u8]) -> bool {
        let address = VMAddress::from_slice(address_bytes);
        &address == self.current_address()
    }

    fn managed_caller(&self, dest_handle: RawHandle) {
        self.m_types_lock()
            .mb_set(dest_handle, self.input_ref().from.to_vec());
    }

    fn managed_sc_address(&self, dest_handle: RawHandle) {
        self.m_types_lock()
            .mb_set(dest_handle, self.current_address().to_vec());
    }

    fn managed_owner_address(&self, dest_handle: RawHandle) {
        self.m_types_lock().mb_set(
            dest_handle,
            self.current_account_data()
                .contract_owner
                .unwrap_or_else(|| panic!("contract owner address not set"))
                .to_vec(),
        );
    }

    fn is_smart_contract(&self, address_bytes: &[u8]) -> bool {
        VMAddress::from_slice(address_bytes).is_smart_contract_address()
    }

    fn load_balance(&self, address_bytes: &[u8], dest: RawHandle) {
        assert!(
            self.is_contract_address(address_bytes),
            "get balance not yet implemented for accounts other than the contract itself"
        );
        self.m_types_lock()
            .bi_overwrite(dest, self.current_account_data().klv_balance.into());
    }

    fn get_tx_hash(&self, dest: RawHandle) {
        self.m_types_lock()
            .mb_set(dest, self.input_ref().tx_hash.to_vec());
    }

    fn get_gas_left(&self) -> u64 {
        self.input_ref().gas_limit
    }

    fn get_block_timestamp(&self) -> u64 {
        self.get_current_block_info().block_timestamp
    }

    fn get_block_nonce(&self) -> u64 {
        self.get_current_block_info().block_nonce
    }

    fn get_block_round(&self) -> u64 {
        self.get_current_block_info().block_round
    }

    fn get_block_epoch(&self) -> u64 {
        self.get_current_block_info().block_epoch
    }

    fn get_block_random_seed(&self, dest: RawHandle) {
        self.m_types_lock().mb_set(
            dest,
            self.get_current_block_info().block_random_seed.to_vec(),
        );
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        self.get_previous_block_info().block_timestamp
    }

    fn get_prev_block_nonce(&self) -> u64 {
        self.get_previous_block_info().block_nonce
    }

    fn get_prev_block_round(&self) -> u64 {
        self.get_previous_block_info().block_round
    }

    fn get_prev_block_epoch(&self) -> u64 {
        self.get_previous_block_info().block_epoch
    }

    fn get_prev_block_random_seed(&self, dest: RawHandle) {
        self.m_types_lock().mb_set(
            dest,
            self.get_previous_block_info().block_random_seed.to_vec(),
        );
    }

    fn big_int_get_kda_external_balance(
        &self,
        address_bytes: &[u8],
        token_id_bytes: &[u8],
        nonce: u64,
        dest: RawHandle,
    ) {
        assert!(
            self.is_contract_address(address_bytes),
            "get_kda_balance not yet implemented for accounts other than the contract itself"
        );

        let kda_balance = self
            .current_account_data()
            .kda
            .get_kda_balance(token_id_bytes, nonce);
        self.m_types_lock().bi_overwrite(dest, kda_balance.into());
    }

    #[allow(clippy::too_many_arguments)]
    fn set_user_kda_values(
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
        // TODO: implement
        todo!()
    }

    #[allow(clippy::too_many_arguments)]
    fn reset_user_kda_values(
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
        todo!()
    }

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
    ) {
        let address = VMAddress::from_slice(self.m_types_lock().mb_get(address_handle));
        let token_id_bytes = self.m_types_lock().mb_get(ticker_handle).to_vec();

        if let Some(account) = self.account_data(&address) {
            if let Some(kda_data) = account.kda.get_by_identifier(token_id_bytes.as_slice()) {
                if let Some(_instance) = kda_data.instances.get_by_nonce(nonce) {
                    self.set_user_kda_values(
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
                    return;
                }
            }
        }
        self.reset_user_kda_values(
            address_handle,
            ticker_handle,
            nonce,
            balance_handle,
            frozen_handle,
            last_claim_handle,
            buckets_handle,
            mime_handle,
            metadata_handle,
        )
    }

    fn managed_get_sft_metadata(
        &self,
        _ticker_handle: RawHandle,
        _nonce: u64,
        _data_handle: RawHandle,
    ) {
        todo!()
    }

    fn managed_acc_has_perm(
        &self,
        ops: i64,
        source_acc_addr: RawHandle,
        target_acc_addr: RawHandle,
    ) -> i32 {
        let source_address = VMAddress::from_slice(self.m_types_lock().mb_get(source_acc_addr));
        let target_address = VMAddress::from_slice(self.m_types_lock().mb_get(target_acc_addr));

        let source_account: AccountData = match self.account_data(&source_address) {
            None => {
                self.vm_error(&format!(
                    "account not found: {}",
                    hex::encode(source_address.as_bytes())
                ));
            },
            Some(src_acc) => src_acc,
        };

        let target_account: AccountData = match self.account_data(&target_address) {
            None => {
                self.vm_error(&format!(
                    "account not found: {}",
                    hex::encode(target_address.as_bytes())
                ));
            },
            Some(tgt_acc) => tgt_acc,
        };

        for perm in source_account.permissions {
            if let Some(signer_address) = perm.address {
                if signer_address == target_account.address
                    && perm.operations & (ops as u64) == (ops as u64)
                {
                    return 1;
                }
            }
        }

        0
    }

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
    ) {
        let address = VMAddress::from_slice(self.m_types_lock().mb_get(address_handle));
        let token_id_bytes = self.m_types_lock().mb_get(ticker_handle).to_vec();

        if let Some(account) = self.account_data(&address) {
            if let Some(kda_data) = account.kda.get_by_identifier(token_id_bytes.as_slice()) {
                if let Some(instance) = kda_data.instances.get_by_nonce(nonce) {
                    self.set_kda_data_values(
                        kda_data,
                        instance,
                        precision_handle,
                        id_handle,
                        name_handle,
                        creator_handle,
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
                    return;
                }
            }
        }
        self.reset_kda_data_values(
            precision_handle,
            id_handle,
            name_handle,
            creator_handle,
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

    fn managed_get_kda_roles(&self, _token_id_handle: i32, _roles_handle: i32) {
        todo!()
    }

    fn managed_get_back_transfers(
        &self,
        kda_transfer_value_handle: RawHandle,
        call_value_handle: RawHandle,
    ) {
        let back_transfers = self.back_transfers_lock();
        let mut m_types = self.m_types_lock();
        m_types.bi_overwrite(call_value_handle, back_transfers.call_value.clone().into());
        m_types
            .mb_set_vec_of_kda_payments(kda_transfer_value_handle, &back_transfers.kda_transfers);
    }

    #[allow(clippy::too_many_arguments)]
    fn set_kda_data_values(
        &self,
        _kda_data: &KdaData,
        instance: &KdaInstance,
        _precision_handle: RawHandle,
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
        properties_handle: RawHandle,
        _attributes_handle: RawHandle,
        _roles_handle: RawHandle,
        _issue_date_handle: RawHandle,
    ) {
        // TODO: maybe need to add other fields here
        let mut m_types = self.m_types_lock();
        let mut prop: u64 = 0;

        if instance.metadata.can_burn {
            prop += 1 << 4;
        }

        m_types.bi_overwrite(properties_handle, BigInt::from(prop));

        // if kda_data.frozen {
        //     m_types.mb_set(properties_handle, vec![1, 0]);
        // } else {
        //     m_types.mb_set(properties_handle, vec![0, 0]);
        // }
        // m_types.mb_set(
        //     hash_handle,
        //     instance.metadata.hash.clone().unwrap_or_default(),
        // );
        // m_types.mb_set(name_handle, instance.metadata.name.clone());
        // m_types.mb_set(attributes_handle, instance.metadata.attributes.clone());
        // if let Some(creator) = &instance.metadata.creator {
        //     m_types.mb_set(creator_handle, creator.to_vec());
        // } else {
        //     m_types.mb_set(creator_handle, vec![0u8; 32]);
        // };
        // m_types.bi_overwrite(
        //     royalties_handle,
        //     num_bigint::BigInt::from(instance.metadata.royalties),
        // );
        // m_types.mb_set_vec_of_bytes(uris_handle, instance.metadata.uri.clone());
    }

    #[allow(clippy::too_many_arguments)]
    fn reset_kda_data_values(
        &self,
        _precision_handle: RawHandle,
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
        if KDA_TOKEN_DATA_FUNC_RESETS_VALUES {
            todo!()
            // TODO: implement
            // let mut m_types = self.m_types_lock();
            // m_types.bi_overwrite(value_handle, BigInt::zero());
            // m_types.mb_set(properties_handle, vec![0, 0]);
            // m_types.mb_set(hash_handle, vec![]);
            // m_types.mb_set(name_handle, vec![]);
            // m_types.mb_set(attributes_handle, vec![]);
            // m_types.mb_set(creator_handle, vec![0u8; 32]);
            // m_types.bi_overwrite(royalties_handle, BigInt::zero());
            // m_types.bi_overwrite(uris_handle, BigInt::zero());
        }
    }

    fn managed_get_code_metadata(&self, address_handle: i32, response_handle: i32) {
        let address = VMAddress::from_slice(self.m_types_lock().mb_get(address_handle));
        let Some(data) = self.account_data(&address) else {
            self.vm_error("account was not found")
        };
        let code_metadata_bytes = data.code_metadata.to_byte_array();
        self.m_types_lock()
            .mb_set(response_handle, code_metadata_bytes.to_vec())
    }

    fn managed_is_builtin_function(&self, function_name_handle: i32) -> bool {
        VM_BUILTIN_FUNCTION_NAMES.contains(
            &self
                .m_types_lock()
                .mb_to_function_name(function_name_handle)
                .as_str(),
        )
    }
}
