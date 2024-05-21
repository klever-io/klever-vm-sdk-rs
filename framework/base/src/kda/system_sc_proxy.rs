use core::marker::PhantomData;

use crate::{
    api::{AssetType, CallTypeApi, SendApi, StorageReadApi},
    contract_base::{BlockchainWrapper, SendWrapper},
    types::{
        BigUint, KdaTokenType, ManagedAddress, ManagedBuffer, ManagedVec, PropertiesInfo,
        RoyaltiesData, TokenIdentifier,
    },
};

/// Proxy for the KDA system smart contract.
/// Unlike other contract proxies, this one has a fixed address,
/// so the proxy object doesn't really contain any data, it is more of a placeholder.
pub struct KDASystemSmartContractProxy<SA>
where
    SA: SendApi + 'static,
{
    _phantom: PhantomData<SA>,
}

impl<SA> KDASystemSmartContractProxy<SA>
where
    SA: SendApi + 'static,
{
    /// Constructor.
    pub fn new_proxy_obj() -> Self {
        KDASystemSmartContractProxy {
            _phantom: PhantomData,
        }
    }
}

impl<SA> KDASystemSmartContractProxy<SA>
where
    SA: StorageReadApi + CallTypeApi + 'static,
{
    fn get_sc_address() -> ManagedAddress<SA> {
        let b_wrapper = BlockchainWrapper::new();
        b_wrapper.get_sc_address()
    }

    /// Produces a contract call to the KDA system SC,
    /// which causes it to issue a new fungible KDA token.
    pub fn issue_fungible(
        self,
        token_display_name: &ManagedBuffer<SA>,
        num_decimals: u32,
        token_ticker: &ManagedBuffer<SA>,
        initial_supply: &BigUint<SA>,
        max_supply: &BigUint<SA>,
        properties: &PropertiesInfo,
    ) -> TokenIdentifier<SA> {
        self.issue(
            KdaTokenType::Fungible,
            token_display_name,
            token_ticker,
            num_decimals,
            initial_supply,
            max_supply,
            properties,
        )
    }

    /// Produces a contract call to the KDA system SC,
    /// which causes it to issue a new non-fungible KDA token.
    pub fn issue_non_fungible(
        self,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        properties: &PropertiesInfo,
    ) -> TokenIdentifier<SA> {
        self.issue(
            KdaTokenType::NonFungible,
            token_display_name,
            token_ticker,
            0,
            &BigUint::zero(),
            &BigUint::zero(),
            properties,
        )
    }

    /// Produces a contract call to the KDA system SC,
    /// which causes it to issue a new semi-fungible KDA token.
    pub fn issue_semi_fungible(
        self,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        num_decimals: u32,
        properties: &PropertiesInfo,
    ) -> TokenIdentifier<SA> {
        self.issue(
            KdaTokenType::NonFungible,
            token_display_name,
            token_ticker,
            num_decimals,
            &BigUint::zero(),
            &BigUint::zero(),
            properties,
        )
    }

    fn issue(
        self,
        token_type: KdaTokenType,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        num_decimals: u32,
        initial_supply: &BigUint<SA>,
        max_supply: &BigUint<SA>,
        properties: &PropertiesInfo,
    ) -> TokenIdentifier<SA> {
        let asset_type = match token_type {
            KdaTokenType::Fungible => AssetType::Fungible,
            KdaTokenType::NonFungible => AssetType::NFT,
            KdaTokenType::SemiFungible => AssetType::SemiFungible,
            _ => panic!("Invalid token type"),
        };

        let send_wrapper = SendWrapper::<SA>::new();
        let result = send_wrapper.kda_create(
            asset_type,
            token_display_name,
            token_ticker,
            num_decimals,
            &Self::get_sc_address(),
            &ManagedBuffer::new(),
            initial_supply,
            max_supply,
            properties,
            &RoyaltiesData::default(),
        );

        result
    }

    /// Produces a contract call to the KDA system SC,
    /// which causes it to mint more fungible KDA tokens.
    /// It will fail if the SC is not the owner of the token.
    pub fn mint(
        self,
        token_identifier: &TokenIdentifier<SA>,
        amount: &BigUint<SA>,
    ) -> ManagedVec<SA, ManagedBuffer<SA>> {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.kda_mint(token_identifier, 0, amount)
    }

    /// Produces a contract call to the KDA system SC,
    /// which causes it to mint more fungible KDA tokens.
    /// It will fail if the SC is not the owner of the token.
    pub fn mint_with_address(
        self,
        to: &ManagedAddress<SA>,
        token_identifier: &TokenIdentifier<SA>,
        amount: &BigUint<SA>,
    ) -> ManagedVec<SA, ManagedBuffer<SA>> {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.kda_mint_with_address(token_identifier, 0, amount, to, 0)
    }

    /// Produces a contract call to the KDA system SC,
    /// which causes it to burn fungible KDA tokens owned by the SC.
    pub fn burn(self, token_identifier: &TokenIdentifier<SA>, amount: &BigUint<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.kda_burn(&token_identifier, 0, &amount);
    }

    /// The manager of an KDA token may choose to suspend all transactions of the token,
    /// except minting, freezing/unfreezing and wiping.
    pub fn pause(self, token_identifier: &TokenIdentifier<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.kda_pause(token_identifier)
    }

    /// The reverse operation of `pause`.
    pub fn unpause(self, token_identifier: &TokenIdentifier<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.kda_resume(&token_identifier)
    }

    /// The manager of an KDA token may freeze the tokens held by a specific account.
    /// As a consequence, no tokens may be transferred to or from the frozen account.
    /// Freezing and unfreezing the tokens of an account are operations designed to help token managers to comply with regulations.
    pub fn freeze(
        self,
        token_identifier: &TokenIdentifier<SA>,
        amount: &BigUint<SA>,
    ) -> ManagedBuffer<SA> {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.freeze(token_identifier, amount)
    }

    /// The reverse operation of `freeze`, unfreezing, will allow further transfers to and from the account.
    pub fn unfreeze(self, token_identifier: &TokenIdentifier<SA>, bucket_id: &ManagedBuffer<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.unfreeze(token_identifier, bucket_id)
    }

    /// The manager of an KDA token may wipe out all the tokens held by a frozen account.
    /// This operation is similar to burning the tokens, but the account must have been frozen beforehand,
    /// and it must be done by the token manager.
    /// Wiping the tokens of an account is an operation designed to help token managers to comply with regulations.
    pub fn wipe(
        self,
        token_identifier: &TokenIdentifier<SA>,
        nonce: u64,
        amount: &BigUint<SA>,
        address: &ManagedAddress<SA>,
    ) {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.kda_wipe(token_identifier, nonce, amount, address)
    }

    /// This function can be called only if owner of the token is the SC.
    /// address will receiver special roles related to the token.
    pub fn set_special_roles(
        self,
        address: &ManagedAddress<SA>,
        token_identifier: &TokenIdentifier<SA>,
        allow_mint: bool,
        alow_set_ito_price: bool,
        allow_deposit: bool,
        allow_transfer: bool,
    ) {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.kda_add_role(
            token_identifier,
            address,
            allow_mint,
            alow_set_ito_price,
            allow_deposit,
            allow_transfer,
        )
    }

    pub fn transfer_ownership(
        self,
        token_identifier: &TokenIdentifier<SA>,
        new_owner: &ManagedAddress<SA>,
    ) {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.kda_change_owner(token_identifier, new_owner)
    }
}
