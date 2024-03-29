use num_bigint::BigInt;

use crate::{
    types::{RawHandle, VMAddress},
    vm_hooks::VMHooksHandlerSource,
    world_mock::{KdaData, KdaInstance},
};
// use num_bigint::BigInt;
// use num_traits::Zero;

// The Go VM doesn't do it, but if we change that, we can enable it easily here too via this constant.
const KDA_TOKEN_DATA_FUNC_RESETS_VALUES: bool = false;

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
    ){
        let address = VMAddress::from_slice(self.m_types_lock().mb_get(address_handle));
        let token_id_bytes = self.m_types_lock().mb_get(ticker_handle).to_vec();
        let account = self.account_data(&address);

        if let Some(kda_data) = account.kda.get_by_identifier(token_id_bytes.as_slice()) {
            if let Some(_instance) = kda_data.instances.get_by_nonce(nonce) {
                self.set_user_kda_values(address_handle, ticker_handle, nonce, balance_handle, frozen_handle, last_claim_handle, buckets_handle, mime_handle, metadata_handle)
            }
            else {
                self.reset_user_kda_values(address_handle, ticker_handle, nonce, balance_handle, frozen_handle, last_claim_handle, buckets_handle, mime_handle, metadata_handle)
            }
        }
        else {
            self.reset_user_kda_values(address_handle, ticker_handle, nonce, balance_handle, frozen_handle, last_claim_handle, buckets_handle, mime_handle, metadata_handle)
        }
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
        let account = self.account_data(&address);

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
                )
            } else {
                // missing nonce
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
        } else {
            // missing token identifier
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
    }

    fn managed_get_kda_roles(
        &self,
        _token_id_handle: i32,
        _roles_handle: i32,
    ) {
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
        m_types.mb_set_vec_of_kda_payments(
            kda_transfer_value_handle,
            &back_transfers.kda_transfers,
        );
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
        _properties_handle: RawHandle,
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

        m_types.bi_overwrite(_properties_handle, BigInt::from(prop));

    
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
}
