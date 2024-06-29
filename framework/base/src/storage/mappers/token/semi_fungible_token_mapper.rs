use super::{
    super::StorageMapper,
    token_mapper::{check_not_set, store_token_id, StorageTokenWrapper, INVALID_TOKEN_ID_ERR_MSG},
    TokenMapperState,
};

use crate::{
    abi::{TypeAbi, TypeName},
    api::{CallTypeApi, ErrorApiImpl, StorageMapperApi},
    codec::{
        CodecFrom, EncodeErrorHandler, TopDecode, TopEncode, TopEncodeMulti, TopEncodeMultiOutput,
    },
    contract_base::{BlockchainWrapper, SendWrapper},
    imports::{KdaTokenPayment, SFTMeta}, kda::KDASystemSmartContractProxy, storage::StorageKey, storage_get,
    types::{
        BigUint, KdaTokenData, ManagedAddress, ManagedBuffer, ManagedType, ManagedVec, PropertiesInfo, TokenIdentifier
    }
};

pub struct SemiFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    key: StorageKey<SA>,
    token_state: TokenMapperState<SA>,
}

impl<SA> StorageMapper<SA> for SemiFungibleTokenMapper<SA>
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

impl<SA> StorageTokenWrapper<SA> for SemiFungibleTokenMapper<SA>
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

impl<SA> SemiFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    pub fn issue(
        &mut self,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        precision: Option<u32>
    ) -> TokenIdentifier<SA> {
        check_not_set(self);

        let system_sc_proxy = KDASystemSmartContractProxy::<SA>::new_proxy_obj();
        let token_id = system_sc_proxy.issue_semi_fungible(
            token_display_name,
            token_ticker,
            precision.unwrap_or(0),
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
        );


        self.set_token_id(token_id.clone());

        token_id
    }

    pub fn sft_mint<T: TopEncode>(
        &self, 
        amount: &BigUint<SA>,
        attributes: &T,
    ) -> u64 {
        let system_sc_proxy = KDASystemSmartContractProxy::<SA>::new_proxy_obj();
        let token_id = self.get_token_id();

        let output = system_sc_proxy.mint(&token_id, &amount);

        let nonce = if let Some(first_result_bytes) = output.try_get(0) {
            first_result_bytes.parse_as_u64().unwrap()
        } else {
            panic!("Failed to get nonce from mint result")
        };

        let mut encoded_buffer = ManagedBuffer::new();
        if let Result::Err(err) = attributes.top_encode(&mut encoded_buffer) {
            panic!("Failed to encode attributes: {}", err.message_str());
        };
        let empty_addr = ManagedAddress::default();
        let empty = ManagedBuffer::new();

        
        system_sc_proxy.update_metadata(&token_id, nonce, &empty_addr, &empty,&encoded_buffer, &ManagedBuffer::new());

        nonce
    }

    pub fn sft_mint_to_address(
        &self,
        to: &ManagedAddress<SA>,
        amount: &BigUint<SA>,
    ) -> ManagedVec<SA, ManagedBuffer<SA>> {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id();

        send_wrapper.kda_mint_with_address(&token_id, 0, amount, to, 0)
    }

    pub fn sft_add_quantity(
        &self, 
        nonce: u64,
        amount: &BigUint<SA>,
    ) -> KdaTokenPayment<SA> {
        let system_sc_proxy = KDASystemSmartContractProxy::<SA>::new_proxy_obj();
        let token_id = self.get_token_id();

        let _ = system_sc_proxy.sft_add_quantity(&token_id, nonce, &amount);

        KdaTokenPayment::new(token_id, nonce, amount.clone())
    }

    pub fn sft_add_quantity_and_send(
        &self, 
        to: &ManagedAddress<SA>,
        nonce: u64,
        amount: &BigUint<SA>,
    ) -> KdaTokenPayment<SA> {
        let payment = self.sft_add_quantity(nonce, amount);

        self.send_payment(to, &payment);

        payment
    }

    fn send_payment(&self, to: &ManagedAddress<SA>, payment: &KdaTokenPayment<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.direct_payment(
            to,
            &payment
        );
    }

    pub fn sft_burn(&self, token_nonce: u64, amount: &BigUint<SA>) {
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

    pub fn get_balance(&self, token_nonce: u64) -> BigUint<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_kda_balance(&own_sc_address, token_id, token_nonce)
    }

    pub fn get_sft_meta(&self, token_nonce: u64) -> SFTMeta<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_sft_metadata(&token_id, token_nonce)
    }

    pub fn get_sft_meta_attributes<T: TopDecode>(&self, token_nonce: u64) -> T {
        let meta = self.get_sft_meta(token_nonce);
        
        meta.metadata.decode_attributes()
    }
}

impl<SA> TopEncodeMulti for SemiFungibleTokenMapper<SA>
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

impl<SA> CodecFrom<SemiFungibleTokenMapper<SA>> for TokenIdentifier<SA> where
    SA: StorageMapperApi + CallTypeApi
{
}

impl<SA> TypeAbi for SemiFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn type_name() -> TypeName {
        TokenIdentifier::<SA>::type_name()
    }

    fn provide_type_descriptions<TDC: crate::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
        TokenIdentifier::<SA>::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        false
    }
}
