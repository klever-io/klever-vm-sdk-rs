use super::{
    super::StorageMapper,
    token_mapper::{check_not_set, store_token_id, StorageTokenWrapper, INVALID_TOKEN_ID_ERR_MSG},
    TokenMapperState,
};

use crate::{
    abi::{TypeAbi, TypeName},
    api::{ErrorApiImpl, AssetType, CallTypeApi, StorageMapperApi},
    codec::{EncodeErrorHandler, TopEncodeMulti, TopEncodeMultiOutput},
    contract_base::{BlockchainWrapper, SendWrapper},
    storage::StorageKey, storage_get,
    types::{
        RoyaltiesData, PropertiesInfo,
        BigUint, KdaTokenPayment, ManagedAddress,
        ManagedBuffer, ManagedType, TokenIdentifier,
    },
};
use crate::abi::TypeAbiFrom;
use crate::types::Tx;

pub struct FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    key: StorageKey<SA>,
    token_state: TokenMapperState<SA>,
}

impl<SA> StorageMapper<SA> for FungibleTokenMapper<SA>
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

impl<SA> StorageTokenWrapper<SA> for FungibleTokenMapper<SA>
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
            SA::error_api_impl().signal_error(INVALID_TOKEN_ID_ERR_MSG)
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

impl<SA> FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    pub fn issue(
        &mut self,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        initial_supply: &BigUint<SA>,
        max_supply: &BigUint<SA>,
        num_decimals: u32,
    ) -> TokenIdentifier<SA> {
        check_not_set(self);

        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = send_wrapper.kda_create(
            AssetType::Fungible, token_display_name, token_ticker, num_decimals, 
            &Self::get_sc_address(),
             &ManagedBuffer::new(), initial_supply, max_supply, &PropertiesInfo::default(), &RoyaltiesData::default(),
        );

        self.set_token_id(token_id.clone());

        token_id
    }

    pub fn mint(&self, amount: &BigUint<SA>) -> KdaTokenPayment<SA> {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id();

        send_wrapper.kda_mint(&token_id, 0, amount);

        KdaTokenPayment::new(token_id, 0, amount.clone())
    }

    pub fn mint_and_send(
        &self,
        to: &ManagedAddress<SA>,
        amount: &BigUint<SA>,
    ) -> KdaTokenPayment<SA> {
        let payment = self.mint(amount);
        self.send_payment(to, &payment);

        payment
    }

    pub fn burn(&self, amount: &BigUint<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id_ref();

        send_wrapper.kda_burn(token_id, 0, amount);
    }

    pub fn get_balance(&self) -> BigUint<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_kda_balance(&own_sc_address, token_id, 0)
    }

    fn send_payment(&self, to: &ManagedAddress<SA>, payment: &KdaTokenPayment<SA>) {
        Tx::new_tx_from_sc()
            .to(to)
            .single_kda(&payment.token_identifier, 0, &payment.amount)
            .transfer();
    }
}

impl<SA> TopEncodeMulti for FungibleTokenMapper<SA>
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

impl<SA> TypeAbiFrom<FungibleTokenMapper<SA>> for TokenIdentifier<SA> where
    SA: StorageMapperApi + CallTypeApi
{
}

impl<SA> TypeAbiFrom<Self> for FungibleTokenMapper<SA> where SA: StorageMapperApi + CallTypeApi {}

impl<SA> TypeAbi for FungibleTokenMapper<SA>
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
