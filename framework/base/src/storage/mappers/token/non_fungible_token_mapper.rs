use super::{
    super::StorageMapper,
    token_mapper::{check_not_set, store_token_id, StorageTokenWrapper, INVALID_TOKEN_ID_ERR_MSG},
    TokenMapperState,
};

use crate::abi::TypeAbiFrom;
use crate::{
    abi::{TypeAbi, TypeName},
    api::{CallTypeApi, ErrorApiImpl, StorageMapperApi},
    codec::{EncodeErrorHandler, TopDecode, TopEncodeMulti, TopEncodeMultiOutput},
    contract_base::{BlockchainWrapper, SendWrapper},
    imports::{AttributesInfo, UserKDA},
    kda::KDASystemSmartContractProxy,
    storage::StorageKey,
    storage_get,
    types::{
        BigUint, KdaTokenData, ManagedAddress, ManagedBuffer, ManagedType, ManagedVec,
        PropertiesInfo, TokenIdentifier, URI,
    },
};

pub struct NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    key: StorageKey<SA>,
    token_state: TokenMapperState<SA>,
}

impl<SA> StorageMapper<SA> for NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self {
            token_state: storage_get(base_key.as_ref()),
            key: base_key,
        }
    }
}

impl<SA> StorageTokenWrapper<SA> for NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn get_storage_key(&self) -> crate::types::ManagedRef<SA, StorageKey<SA>> {
        self.key.as_ref()
    }

    fn get_token_state(&self) -> TokenMapperState<SA> {
        self.token_state.clone()
    }

    fn get_token_id(&self) -> TokenIdentifier<SA> {
        if let TokenMapperState::Token(token) = &self.token_state {
            token.clone()
        } else {
            SA::error_api_impl().signal_error(INVALID_TOKEN_ID_ERR_MSG);
        }
    }

    fn get_token_id_ref(&self) -> &TokenIdentifier<SA> {
        if let TokenMapperState::Token(token) = &self.token_state {
            token
        } else {
            SA::error_api_impl().signal_error(INVALID_TOKEN_ID_ERR_MSG);
        }
    }

    fn set_token_id(&mut self, token_id: TokenIdentifier<SA>) {
        store_token_id(self, &token_id);
        self.token_state = TokenMapperState::Token(token_id);
    }
}

impl<SA> NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    pub fn issue(
        &mut self,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
    ) -> TokenIdentifier<SA> {
        check_not_set(self);

        let system_sc_proxy = KDASystemSmartContractProxy::<SA>::new_proxy_obj();
        let token_id = system_sc_proxy.issue_non_fungible(
            token_display_name,
            token_ticker,
            &PropertiesInfo {
                can_freeze: false,
                can_wipe: false,
                can_pause: true,
                can_mint: true,
                can_burn: true,
                can_change_owner: true,
                can_add_roles: true,
                limit_transfer: false,
            },
            &AttributesInfo {
                is_paused: false,
                is_nft_mint_stopped: false,
                is_nft_metadata_change_stopped: false,
                is_royalties_change_stopped: false,
            },
            &ManagedVec::<SA, URI<SA>>::new(),
        );

        self.set_token_id(token_id.clone());

        token_id
    }

    pub fn nft_mint(&self, amount: &BigUint<SA>) -> ManagedVec<SA, ManagedBuffer<SA>> {
        let system_sc_proxy = KDASystemSmartContractProxy::<SA>::new_proxy_obj();
        let token_id = self.get_token_id();

        system_sc_proxy.mint(&token_id, amount)
    }

    pub fn nft_mint_to_address(
        &self,
        to: &ManagedAddress<SA>,
        amount: &BigUint<SA>,
    ) -> ManagedVec<SA, ManagedBuffer<SA>> {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id();

        send_wrapper.kda_mint_with_address(&token_id, 0, amount, to, 0)
    }

    pub fn nft_burn(&self, token_nonce: u64, amount: &BigUint<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id_ref();

        send_wrapper.kda_burn(token_id, token_nonce, amount);
    }

    pub fn get_all_token_data(&self, token_nonce: u64) -> KdaTokenData<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_kda_token_data(&own_sc_address, token_id, token_nonce)
    }

    pub fn get_nft_token_data(&self, token_nonce: u64) -> UserKDA<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_user_kda(&own_sc_address, token_id, token_nonce)
    }

    pub fn get_balance(&self, token_nonce: u64) -> BigUint<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_kda_balance(&own_sc_address, token_id, token_nonce)
    }

    pub fn get_token_attributes<T: TopDecode>(&self, token_nonce: u64) -> AttributesInfo {
        let token_data = self.get_all_token_data(token_nonce);
        token_data.attributes
    }
}

impl<SA> TopEncodeMulti for NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        if self.is_empty() {
            output.push_single_value(&ManagedBuffer::<SA>::new(), h)
        } else {
            output.push_single_value(&self.get_token_id(), h)
        }
    }
}

impl<SA> TypeAbiFrom<NonFungibleTokenMapper<SA>> for TokenIdentifier<SA> where
    SA: StorageMapperApi + CallTypeApi
{
}

impl<SA> TypeAbiFrom<Self> for NonFungibleTokenMapper<SA> where SA: StorageMapperApi + CallTypeApi {}

impl<SA> TypeAbi for NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TokenIdentifier::<SA>::type_name()
    }

    fn type_name_rust() -> TypeName {
        TokenIdentifier::<SA>::type_name_rust()
    }

    fn provide_type_descriptions<TDC: crate::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
        TokenIdentifier::<SA>::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        false
    }
}
