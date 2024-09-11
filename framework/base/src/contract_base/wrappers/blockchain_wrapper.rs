use core::marker::PhantomData;

use crate::types::CodeMetadata;
use crate::{
    api::{
        const_handles, use_raw_handle, BigIntApiImpl, BlockchainApi, BlockchainApiImpl, ErrorApi,
        ErrorApiImpl, HandleConstraints, ManagedBufferApiImpl, ManagedTypeApi, StaticVarApiImpl,
    },
    err_msg::{ONLY_OWNER_CALLER, ONLY_USER_ACCOUNT_CALLER},
    types::{
        convert_buff_to_roles, AttributesInfo, BackTransfers, BigUint, KdaTokenData, KdaTokenType,
        LastClaim, ManagedAddress, ManagedBuffer, ManagedByteArray, ManagedType, ManagedVec,
        PropertiesInfo, RolesInfo, RoyaltiesData, SFTMeta, TokenIdentifier, UserKDA,
    },
};

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
#[derive(Default)]
pub struct BlockchainWrapper<A>
where
    A: BlockchainApi + ManagedTypeApi + ErrorApi,
{
    _phantom: PhantomData<A>,
}

impl<A> BlockchainWrapper<A>
where
    A: BlockchainApi + ManagedTypeApi + ErrorApi,
{
    pub fn new() -> Self {
        BlockchainWrapper {
            _phantom: PhantomData,
        }
    }

    #[deprecated(since = "0.41.0", note = "Please use method `get_caller` instead.")]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_caller_legacy(&self) -> crate::types::Address {
        A::blockchain_api_impl().get_caller_legacy()
    }

    #[inline]
    pub fn get_caller(&self) -> ManagedAddress<A> {
        let handle: A::ManagedBufferHandle = use_raw_handle(A::static_var_api_impl().next_handle());
        A::blockchain_api_impl().load_caller_managed(handle.clone());
        ManagedAddress::from_handle(handle)
    }

    #[deprecated(since = "0.41.0", note = "Please use method `get_sc_address` instead.")]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_sc_address_legacy(&self) -> crate::types::Address {
        A::blockchain_api_impl().get_sc_address_legacy()
    }

    #[inline]
    pub fn get_sc_address(&self) -> ManagedAddress<A> {
        let handle: A::ManagedBufferHandle = use_raw_handle(A::static_var_api_impl().next_handle());
        A::blockchain_api_impl().load_sc_address_managed(handle.clone());
        ManagedAddress::from_handle(handle)
    }

    #[inline]
    pub fn get_owner_address(&self) -> ManagedAddress<A> {
        let handle: A::ManagedBufferHandle = use_raw_handle(A::static_var_api_impl().next_handle());
        A::blockchain_api_impl().load_owner_address_managed(handle.clone());
        ManagedAddress::from_handle(handle)
    }

    pub fn check_caller_is_owner(&self) {
        if self.get_owner_address() != self.get_caller() {
            A::error_api_impl().signal_error(ONLY_OWNER_CALLER);
        }
    }

    pub fn check_caller_is_user_account(&self) {
        let mbuf_temp_1: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::blockchain_api_impl().load_caller_managed(mbuf_temp_1.clone());
        if A::blockchain_api_impl().is_smart_contract(mbuf_temp_1) {
            A::error_api_impl().signal_error(ONLY_USER_ACCOUNT_CALLER);
        }
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `is_smart_contract` instead."
    )]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn is_smart_contract_legacy(&self, address: &crate::types::Address) -> bool {
        A::blockchain_api_impl().is_smart_contract_legacy(address)
    }

    #[inline]
    pub fn is_smart_contract(&self, address: &ManagedAddress<A>) -> bool {
        A::blockchain_api_impl().is_smart_contract(address.get_handle())
    }

    #[deprecated(since = "0.41.0", note = "Please use method `get_balance` instead.")]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_balance_legacy(&self, address: &crate::types::Address) -> BigUint<A> {
        let handle: A::BigIntHandle = use_raw_handle(A::static_var_api_impl().next_handle());
        A::blockchain_api_impl().load_balance_legacy(handle.clone(), address);
        BigUint::from_handle(handle)
    }

    #[inline]
    pub fn get_balance(&self, address: &ManagedAddress<A>) -> BigUint<A> {
        let handle: A::BigIntHandle = use_raw_handle(A::static_var_api_impl().next_handle());
        A::blockchain_api_impl().load_balance(handle.clone(), address.get_handle());
        BigUint::from_handle(handle)
    }

    #[inline]
    pub fn get_code_metadata(&self, address: &ManagedAddress<A>) -> CodeMetadata {
        let mbuf_temp_1: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::blockchain_api_impl()
            .managed_get_code_metadata(address.get_handle(), mbuf_temp_1.clone());
        let mut buffer = [0u8; 2];
        ManagedBuffer::<A>::from_handle(mbuf_temp_1).load_to_byte_array(&mut buffer);
        CodeMetadata::from(buffer)
    }

    #[inline]
    pub fn is_builtin_function(&self, function_name: &ManagedBuffer<A>) -> bool {
        A::blockchain_api_impl().managed_is_builtin_function(function_name.get_handle())
    }

    #[inline]
    pub fn get_sc_balance(&self, token_identifier: &TokenIdentifier<A>, nonce: u64) -> BigUint<A> {
        self.get_kda_balance(&self.get_sc_address(), token_identifier, nonce)
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `get_state_root_hash` instead."
    )]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_state_root_hash_legacy(&self) -> crate::types::H256 {
        self.get_state_root_hash().to_byte_array().into()
    }

    #[inline]
    pub fn get_state_root_hash(&self) -> ManagedByteArray<A, 32> {
        let handle: A::ManagedBufferHandle = use_raw_handle(A::static_var_api_impl().next_handle());
        A::blockchain_api_impl().load_state_root_hash_managed(handle.clone());
        ManagedByteArray::from_handle(handle)
    }

    #[deprecated(since = "0.41.0", note = "Please use method `get_tx_hash` instead.")]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_tx_hash_legacy(&self) -> crate::types::H256 {
        A::blockchain_api_impl().get_tx_hash_legacy()
    }

    #[inline]
    pub fn get_tx_hash(&self) -> ManagedByteArray<A, 32> {
        let handle: A::ManagedBufferHandle = use_raw_handle(A::static_var_api_impl().next_handle());
        A::blockchain_api_impl().load_tx_hash_managed(handle.clone());
        ManagedByteArray::from_handle(handle)
    }

    #[inline]
    pub fn get_gas_left(&self) -> u64 {
        A::blockchain_api_impl().get_gas_left()
    }

    #[inline]
    pub fn get_block_timestamp(&self) -> u64 {
        A::blockchain_api_impl().get_block_timestamp()
    }

    #[inline]
    pub fn get_block_nonce(&self) -> u64 {
        A::blockchain_api_impl().get_block_nonce()
    }

    #[inline]
    pub fn get_block_round(&self) -> u64 {
        A::blockchain_api_impl().get_block_round()
    }

    #[inline]
    pub fn get_block_epoch(&self) -> u64 {
        A::blockchain_api_impl().get_block_epoch()
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `get_block_random_seed` instead."
    )]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_block_random_seed_legacy(&self) -> crate::types::Box<[u8; 48]> {
        crate::types::Box::new(self.get_block_random_seed().to_byte_array())
    }

    #[inline]
    pub fn get_block_random_seed(&self) -> ManagedByteArray<A, 48> {
        let handle: A::ManagedBufferHandle = use_raw_handle(A::static_var_api_impl().next_handle());
        A::blockchain_api_impl().load_block_random_seed_managed(handle.clone());
        ManagedByteArray::from_handle(handle)
    }

    #[inline]
    pub fn get_prev_block_timestamp(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_timestamp()
    }

    #[inline]
    pub fn get_prev_block_nonce(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_nonce()
    }

    #[inline]
    pub fn get_prev_block_round(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_round()
    }

    #[inline]
    pub fn get_prev_block_epoch(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_epoch()
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `get_prev_block_random_seed` instead."
    )]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_prev_block_random_seed_legacy(&self) -> crate::types::Box<[u8; 48]> {
        A::blockchain_api_impl().get_prev_block_random_seed_legacy()
    }

    #[inline]
    pub fn get_prev_block_random_seed(&self) -> ManagedByteArray<A, 48> {
        let handle: A::ManagedBufferHandle = use_raw_handle(A::static_var_api_impl().next_handle());
        A::blockchain_api_impl().load_prev_block_random_seed_managed(handle.clone());
        ManagedByteArray::from_handle(handle)
    }

    #[inline]
    pub fn get_kda_balance(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
    ) -> BigUint<A> {
        // KLV balance is not stored in KDA data.
        // It is stored in the account data, so we need to handle balance retrieval separately.
        if token_id.is_klv() {
            return self.get_balance(address);
        }
        let result_handle: A::BigIntHandle = use_raw_handle(A::static_var_api_impl().next_handle());
        A::blockchain_api_impl().load_kda_balance(
            address.get_handle(),
            token_id.get_handle(),
            nonce,
            result_handle.clone(),
        );
        BigUint::from_handle(result_handle)
    }

    pub fn get_user_kda(
        &self,
        address: &ManagedAddress<A>,
        ticker: &TokenIdentifier<A>,
        nonce: u64,
    ) -> UserKDA<A> {
        let managed_api_impl = A::managed_type_impl();

        let balance_handle = managed_api_impl.bi_new_zero();
        let frozen_handle = managed_api_impl.bi_new_zero();
        let last_claim_handle = managed_api_impl.mb_new_empty();
        let buckets_handle = managed_api_impl.mb_new_empty();
        let mime_handle = managed_api_impl.mb_new_empty();
        let metadata_handle = managed_api_impl.mb_new_empty();

        A::blockchain_api_impl().managed_get_user_kda(
            address.get_handle().get_raw_handle(),
            ticker.get_handle().get_raw_handle(),
            nonce,
            balance_handle.get_raw_handle(),
            frozen_handle.get_raw_handle(),
            last_claim_handle.get_raw_handle(),
            buckets_handle.get_raw_handle(),
            mime_handle.get_raw_handle(),
            metadata_handle.get_raw_handle(),
        );

        let mut balance = BigUint::from_raw_handle(balance_handle.get_raw_handle());
        // KLV balance is not stored in KDA data.
        // It is stored in the account data, so we need to handle balance retrieval separately.
        if ticker.is_klv() {
            balance = self.get_balance(address)
        }

        UserKDA {
            balance,
            frozen_balance: BigUint::from_raw_handle(frozen_handle.get_raw_handle()),
            last_claim: LastClaim::from(ManagedBuffer::from_raw_handle(
                last_claim_handle.get_raw_handle(),
            )),
            buckets: ManagedVec::from_raw_handle(buckets_handle.get_raw_handle()),
            mime: ManagedBuffer::from_raw_handle(mime_handle.get_raw_handle()),
            metadata: ManagedBuffer::from_raw_handle(metadata_handle.get_raw_handle()),
        }
    }

    pub fn get_sft_metadata(&self, ticker: &TokenIdentifier<A>, nonce: u64) -> SFTMeta<A> {
        let managed_api_impl = A::managed_type_impl();
        let data_handle = managed_api_impl.mb_new_empty();

        A::blockchain_api_impl().managed_get_sft_metadata(
            ticker.get_handle().get_raw_handle(),
            nonce,
            data_handle.get_raw_handle(),
        );

        SFTMeta::from(ManagedBuffer::from_raw_handle(data_handle.get_raw_handle()))
    }

    pub fn get_kda_token_data(
        &self,
        address: &ManagedAddress<A>,
        ticker: &TokenIdentifier<A>,
        nonce: u64,
    ) -> KdaTokenData<A> {
        // initializing outputs
        // the current version of VM does not set/overwrite them if the token is missing,
        // which is why we need to initialize them explicitly
        let managed_api_impl = A::managed_type_impl();

        let id_handle = managed_api_impl.mb_new_empty();
        let name_handle = managed_api_impl.mb_new_empty();
        let creator_handle = managed_api_impl.mb_new_empty();
        let admin_handle = managed_api_impl.mb_new_empty();
        let logo_handle = managed_api_impl.mb_new_empty();
        let uris_handle = managed_api_impl.mb_new_empty();
        let initial_supply_handle = managed_api_impl.bi_new_zero();
        let circulating_supply_handle = managed_api_impl.bi_new_zero();
        let max_supply_handle = managed_api_impl.bi_new_zero();
        let minted_handle = managed_api_impl.bi_new_zero();
        let burned_handle = managed_api_impl.bi_new_zero();
        let precision_handle = managed_api_impl.bi_new_zero();
        let royalties_handle = managed_api_impl.mb_new_empty();
        let properties_handle = managed_api_impl.bi_new_zero();
        let attributes_handle = managed_api_impl.bi_new_zero();
        let issue_date_handle = managed_api_impl.bi_new_zero();
        let roles_handle = managed_api_impl.mb_new_empty();

        A::blockchain_api_impl().managed_get_kda_token_data(
            address.get_handle().get_raw_handle(),
            ticker.get_handle().get_raw_handle(),
            nonce,
            precision_handle.get_raw_handle(),
            id_handle.get_raw_handle(),
            name_handle.get_raw_handle(),
            creator_handle.get_raw_handle(),
            admin_handle.get_raw_handle(),
            logo_handle.get_raw_handle(),
            uris_handle.get_raw_handle(),
            initial_supply_handle.get_raw_handle(),
            circulating_supply_handle.get_raw_handle(),
            max_supply_handle.get_raw_handle(),
            minted_handle.get_raw_handle(),
            burned_handle.get_raw_handle(),
            royalties_handle.get_raw_handle(),
            properties_handle.get_raw_handle(),
            attributes_handle.get_raw_handle(),
            roles_handle.get_raw_handle(),
            issue_date_handle.get_raw_handle(),
        );

        if managed_api_impl.mb_len(creator_handle.clone()) == 0 {
            // write 32bytes 0 if no creator handle is set
            managed_api_impl.mb_overwrite(creator_handle.clone(), &[0u8; 32][..]);
        }

        let properties_bi = BigUint::<A>::from_raw_handle(properties_handle.get_raw_handle());

        // get tokenType from properties bits 30 and 31
        let properties_value = match properties_bi.to_u64() {
            Some(value) => value,
            None => A::error_api_impl().signal_error(b"Invalid value for Properties Handler."),
        };

        let type_value = (properties_value >> 30) & 0b11;
        let token_type = match type_value {
            0 => KdaTokenType::Fungible,
            1 => KdaTokenType::NonFungible,
            2 => KdaTokenType::SemiFungible,
            _ => KdaTokenType::Invalid,
        };

        KdaTokenData {
            asset_type: token_type,
            id: ManagedBuffer::from_raw_handle(id_handle.get_raw_handle()),
            name: ManagedBuffer::from_raw_handle(name_handle.get_raw_handle()),
            ticker: ManagedBuffer::from_raw_handle(ticker.get_handle().get_raw_handle()),
            owner_address: ManagedAddress::from_raw_handle(creator_handle.get_raw_handle()),
            admin_address: ManagedAddress::from_raw_handle(admin_handle.get_raw_handle()),
            logo: ManagedBuffer::from_raw_handle(logo_handle.get_raw_handle()),
            precision: BigUint::from_raw_handle(precision_handle.get_raw_handle()),
            initial_supply: BigUint::from_raw_handle(initial_supply_handle.get_raw_handle()),
            circulating_supply: BigUint::from_raw_handle(
                circulating_supply_handle.get_raw_handle(),
            ),
            max_supply: BigUint::from_raw_handle(max_supply_handle.get_raw_handle()),
            minted_value: BigUint::from_raw_handle(minted_handle.get_raw_handle()),
            burned_value: BigUint::from_raw_handle(burned_handle.get_raw_handle()),
            issue_date: BigUint::from_raw_handle(issue_date_handle.get_raw_handle()),
            royalties: RoyaltiesData::from(ManagedBuffer::from_raw_handle(
                royalties_handle.get_raw_handle(),
            )),
            properties: PropertiesInfo::from(properties_value),
            attributes: AttributesInfo::from(BigUint::<A>::from_raw_handle(
                attributes_handle.get_raw_handle(),
            )),
            uris: ManagedVec::from_raw_handle(uris_handle.get_raw_handle()),
            roles: convert_buff_to_roles(ManagedBuffer::from_raw_handle(
                roles_handle.get_raw_handle(),
            )),
        }
    }

    /// `source_acc_addr` will be the one that this function will check if it has the permission specified in the other parameters for `target_acc_addr`
    pub fn acc_has_perm(
        &self,
        ops: i64,
        source_acc_addr: &ManagedAddress<A>,
        target_acc_addr: &ManagedAddress<A>,
    ) -> bool {
        A::blockchain_api_impl().managed_acc_has_perm(
            ops,
            source_acc_addr.get_handle().get_raw_handle(),
            target_acc_addr.get_handle().get_raw_handle(),
        )
    }

    pub fn get_kda_roles(&self, ticker: &TokenIdentifier<A>) -> ManagedVec<A, RolesInfo<A>> {
        // initializing outputs
        // the current version of VM does not set/overwrite them if the token is missing,
        // which is why we need to initialize them explicitly
        let managed_api_impl = A::managed_type_impl();
        let roles_handle = managed_api_impl.mb_new_empty();

        A::blockchain_api_impl().managed_get_kda_roles(
            ticker.get_handle().get_raw_handle(),
            roles_handle.get_raw_handle(),
        );

        convert_buff_to_roles(ManagedBuffer::from_raw_handle(
            roles_handle.get_raw_handle(),
        ))
    }

    pub fn get_kda_properties(&self, ticker: &TokenIdentifier<A>) -> PropertiesInfo {
        let kda_data = self.get_kda_token_data(&self.get_sc_address(), ticker, 0);
        kda_data.properties
    }

    /// Retrieves back-transfers from the VM, after a contract call.
    pub fn get_back_transfers(&self) -> BackTransfers<A> {
        let kda_transfer_value_handle: A::BigIntHandle =
            use_raw_handle(A::static_var_api_impl().next_handle());
        let call_value_handle: A::BigIntHandle =
            use_raw_handle(A::static_var_api_impl().next_handle());

        A::blockchain_api_impl().managed_get_back_transfers(
            kda_transfer_value_handle.get_raw_handle(),
            call_value_handle.get_raw_handle(),
        );

        BackTransfers {
            klv_amount: BigUint::from_raw_handle(call_value_handle.get_raw_handle()),
            kda_payments: ManagedVec::from_raw_handle(kda_transfer_value_handle.get_raw_handle()),
        }
    }

    /// Retrieves and deserializes token attributes from the SC account, with given token identifier and nonce.
    pub fn get_token_attributes(&self, token_id: &TokenIdentifier<A>) -> AttributesInfo {
        let kda_data = self.get_kda_token_data(&self.get_sc_address(), token_id, 0);
        kda_data.attributes
    }
}
