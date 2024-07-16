use core::marker::PhantomData;

use crate::{
    api::{
        AssetTriggerType, AssetType, BlockchainApi, BlockchainApiImpl, BuyType, CallTypeApi,
        ClaimType, DepositType, ErrorApiImpl, ITOStatus, ITOTriggerType, ITOWhitelistStatus,
        SellType, StakingType, StorageReadApi, VoteType, WithdrawType,
        CHANGE_OWNER_BUILTIN_FUNC_NAME, KLEVER_ASSET_TRIGGER_FUNC_NAME, KLEVER_BUY_FUNC_NAME,
        KLEVER_CANCEL_MARKET_ORDER_FUNC_NAME, KLEVER_CLAIM_FUNC_NAME, KLEVER_CONFIG_ITO_FUNC_NAME,
        KLEVER_CONFIG_MARKETPLACE_FUNC_NAME, KLEVER_CREATE_ASSET_FUNC_NAME,
        KLEVER_CREATE_MARKETPLACE_FUNC_NAME, KLEVER_DELEGATE_FUNC_NAME, KLEVER_DEPOSIT_FUNC_NAME,
        KLEVER_FREEZE_FUNC_NAME, KLEVER_ITO_TRIGGER_FUNC_NAME, KLEVER_SELL_FUNC_NAME,
        KLEVER_SET_ACCOUNT_NAME_FUNC_NAME, KLEVER_UNDELEGATE_FUNC_NAME, KLEVER_UNFREEZE_FUNC_NAME,
        KLEVER_UPDATE_ACCOUNT_PERMISSION, KLEVER_VOTE_FUNC_NAME, KLEVER_WITHDRAW_FUNC_NAME,
    },
    codec::{NestedEncode},
    kda::KDASystemSmartContractProxy,
    types::{
        AccountPermission, BigUint, ContractCall, ContractCallNoPayment, ITOPackInfo, ITOWhitelist,
        KdaTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer, ManagedVec,
        PropertiesInfo, RoyaltiesData, TokenIdentifier, URI,
    },
};
use crate::contract_base::{BlockchainWrapper, SendRawWrapper};
use crate::types::{GasLeft, ReturnsRawResult, ToSelf, Tx};

/// API that groups methods that either send KLV or KDA, or that call other contracts.
// pub trait SendApi: Clone + Sized {

#[derive(Default)]
pub struct SendWrapper<A>
where
    A: CallTypeApi + StorageReadApi + BlockchainApi,
{
    _phantom: PhantomData<A>,
}

impl<A> SendWrapper<A>
where
    A: CallTypeApi + StorageReadApi + BlockchainApi,
{
    pub fn new() -> Self {
        SendWrapper {
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn send_raw_wrapper(&self) -> SendRawWrapper<A> {
        SendRawWrapper::new()
    }

    /// A proxy for calling the system smart contract.
    ///
    /// Use the methods of this proxy to launch contract calls to the system SC.
    #[inline]
    pub fn kda_system_sc_proxy(&self) -> KDASystemSmartContractProxy<A> {
        KDASystemSmartContractProxy::new_proxy_obj()
    }

    /// Convenient way to quickly instance a minimal contract call (with no KLV, no arguments, etc.)
    ///
    /// You can further configure this contract call by chaining methods to it.
    #[inline]
    pub fn contract_call<R>(
        &self,
        to: ManagedAddress<A>,
        endpoint_name: impl Into<ManagedBuffer<A>>,
    ) -> ContractCallNoPayment<A, R> {
        ContractCallNoPayment::new(to, endpoint_name)
    }

    /// Sends a single KDA transfer, and calls an endpoint at the destination.
    ///
    /// Avoid if possible, use a contract call with KDA transfer instead, and call `.transfer_execute()` on it.
    #[allow(clippy::too_many_arguments)]
    pub fn direct_kda_with_gas_limit<D>(
        &self,
        to: &ManagedAddress<A>,
        token_identifier: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas: u64,
        endpoint_name: D,
        arguments: &[ManagedBuffer<A>],
    ) where
        D: Into<ManagedBuffer<A>>,
    {
        let _ = self.send_raw_wrapper().transfer_kda_nft_execute(
            to,
            token_identifier,
            nonce,
            amount,
            gas,
            &endpoint_name.into(),
            &arguments.into(),
        );
    }

    /// Sends a single KDA transfer to target address.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn direct_klv(&self, to: &ManagedAddress<A>, amount: &BigUint<A>) {
        Tx::new_tx_from_sc().to(to).klv(amount).transfer();
    }

    /// Sends a single KDA transfer to target address.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn direct_kda(
        &self,
        to: &ManagedAddress<A>,
        token_identifier: &TokenIdentifier<A>,
        token_nonce: u64,
        amount: &BigUint<A>,
    ) {
        Tx::new_tx_from_sc()
            .to(to)
            .single_kda(token_identifier, token_nonce, amount)
            .transfer();
    }

    /// Sends multiple KDA tokens to a target address.
    pub fn direct_multi(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, KdaTokenPayment<A>>,
    ) {
        Tx::new_tx_from_sc().to(to).payment(payments).transfer();
    }

    /// Sends payment to a target address.
    pub fn direct_payment(
        &self,
        to: &ManagedAddress<A>,
        payment: &KdaTokenPayment<A>,
    ) {
        self.direct_kda(
            to,
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        )
    }

    /// Creates a call to the `ChangeOwnerAddress` builtin function.
    pub fn change_owner_address(
        &self,
        child_sc_address: ManagedAddress<A>,
        new_owner: &ManagedAddress<A>,
    ) -> ContractCallNoPayment<A, ()> {
        self.contract_call(child_sc_address, CHANGE_OWNER_BUILTIN_FUNC_NAME)
            .argument(&new_owner)
    }

    /// Allows synchronously calling a local function by name. Execution is resumed afterwards.
    /// You should never have to call this function directly.
    /// Use the other specific methods instead.
    pub fn call_kda_built_in_function(
        &self,
        gas: u64,
        endpoint_name: ManagedBuffer<A>,
        arg_buffer: ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(gas)
            .raw_call(endpoint_name)
            .arguments_raw(arg_buffer)
            .returns(ReturnsRawResult)
            .sync_call()
    }
    
    fn call_local_kda_built_in_function_minimal(
        &self,
        function_name: &str,
        arg_buffer: ManagedArgBuffer<A>,
    ) {
        Tx::new_tx_from_sc()
            .to(ToSelf)
            .gas(GasLeft)
            .raw_call(function_name)
            .arguments_raw(arg_buffer)
            .sync_call()
    }   

    /// Allows synchronous minting of KDA/NFT/SFT (depending on nonce). Execution is resumed afterwards.
    ///
    /// Note that the SC must have the KDALocalMint or KDANftAddQuantity roles set,
    /// or this will fail with "action is not allowed".
    ///
    /// For NFT use nonce 0 and SFT specify the nonce of the SFT.
    pub fn kda_mint(
        &self,
        token: &TokenIdentifier<A>,
        nonce: u64, // For Fungibles and SFT only
        amount: &BigUint<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let b_wrapper = BlockchainWrapper::new();
        let own_address = b_wrapper.get_sc_address();

        self.kda_mint_with_address(token, nonce, amount, &own_address, 0)
    }

    pub fn sft_mint(
        &self,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        max_supply: u64,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let b_wrapper = BlockchainWrapper::new();
        let own_address = b_wrapper.get_sc_address();

        self.kda_mint_with_address(token, nonce, amount, &own_address, max_supply)
    }

    pub fn sft_mint_with_address(
        &self,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        address: &ManagedAddress<A>,
        max_supply: u64,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        self.kda_mint_with_address(token, nonce, amount, &address, max_supply)
    }

    pub fn kda_mint_with_address(
        &self,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        address: &ManagedAddress<A>,
        value: u64,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::Mint as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(nonce);
        arg_buffer.push_arg(amount);
        arg_buffer.push_arg(address);
        arg_buffer.push_arg(value);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(), 
            ManagedBuffer::from(func_name), 
            arg_buffer
        )
    }

    /// Allows synchronous burning of KDA/NFT (depending on nonce). Execution is resumed afterwards.
    ///
    /// Note that the SC must have the KDALocalBurn or KDANftBurn roles set,
    /// or this will fail with "action is not allowed".
    pub fn kda_burn(&self, token: &TokenIdentifier<A>, nonce: u64, amount: &BigUint<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::Burn as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(nonce);
        arg_buffer.push_arg(amount);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    pub fn account_update_permission(
        &self,
        address: &ManagedAddress<A>,
        updates: &ManagedVec<A, AccountPermission<A>>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_UPDATE_ACCOUNT_PERMISSION;

        let mut updates_bytes = ManagedBuffer::<A>::new();
        let _ = updates.dep_encode(&mut updates_bytes);

        arg_buffer.push_arg(address);
        arg_buffer.push_arg(updates_bytes);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows burning of multiple KDA tokens at once.
    ///
    /// Will execute a synchronous call to the appropriate burn builtin function for each.
    pub fn kda_burn_multi(&self, payments: &ManagedVec<A, KdaTokenPayment<A>>) {
        for payment in payments {
            self.kda_burn(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );
        }
    }

    /// Allows synchronous burning of KDA/NFT (depending on nonce) on a respective address. Execution is resumed afterwards.
    ///
    /// Note that the SC must have the KDALocalBurn or KDANftBurn roles set,
    /// or this will fail with "action is not allowed".
    pub fn kda_wipe(
        &self,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        address: &ManagedAddress<A>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::Wipe as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(nonce);
        arg_buffer.push_arg(amount);
        arg_buffer.push_arg(address);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous pause of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_pause(&self, token: &TokenIdentifier<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::Pause as u32);
        arg_buffer.push_arg(token);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous resume of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_resume(&self, token: &TokenIdentifier<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::Resume as u32);
        arg_buffer.push_arg(token);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous change owner of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_change_owner(&self, token: &TokenIdentifier<A>, address: &ManagedAddress<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::ChangeOwner as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(address);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous change admin of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_change_admin(&self, token: &TokenIdentifier<A>, address: &ManagedAddress<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::ChangeAdmin as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(address);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous add a role of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_add_role(
        &self,
        token: &TokenIdentifier<A>,
        address: &ManagedAddress<A>,
        has_role_mint: bool,
        has_role_set_ito_prices: bool,
        has_role_deposit: bool,
        has_role_transfer: bool,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::AddRole as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(address);
        arg_buffer.push_arg(has_role_mint);
        arg_buffer.push_arg(has_role_set_ito_prices);
        arg_buffer.push_arg(has_role_deposit);
        arg_buffer.push_arg(has_role_transfer);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous remove a role of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_remove_role(&self, token: &TokenIdentifier<A>, address: &ManagedAddress<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::RemoveRole as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(address);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous change owner of NFT. Execution is resumed afterwards.
    pub fn kda_update_metadata(
        &self,
        token: &TokenIdentifier<A>,
        nonce: u64,
        address: &ManagedAddress<A>,
        mime: &ManagedBuffer<A>,
        metadata: &ManagedBuffer<A>,
        name: &ManagedBuffer<A>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::UpdateMetadata as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(nonce);
        arg_buffer.push_arg(address);
        arg_buffer.push_arg(mime);
        arg_buffer.push_arg(metadata);
        arg_buffer.push_arg(name);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous stop nft mint of NFT. Execution is resumed afterwards.
    pub fn kda_stop_nft_mint(&self, token: &TokenIdentifier<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::StopNFTMint as u32);
        arg_buffer.push_arg(token);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous update logo of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_update_logo(&self, token: &TokenIdentifier<A>, logo: &ManagedBuffer<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::UpdateLogo as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(logo);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous update uris of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_update_uris(&self, token: &TokenIdentifier<A>, uris: &ManagedVec<A, URI<A>>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::UpdateURIs as u32);
        arg_buffer.push_arg(token);

        let mut uri_bytes = ManagedBuffer::<A>::new();
        let _ = uris.dep_encode(&mut uri_bytes);
        arg_buffer.push_arg(uri_bytes);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous change royalties receiver of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_change_royalties_receiver(
        &self,
        token: &TokenIdentifier<A>,
        address: &ManagedAddress<A>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::ChangeRoyaltiesReceiver as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(address);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous update staking of KDA. Execution is resumed afterwards.
    pub fn kda_update_staking(
        &self,
        token: &TokenIdentifier<A>,
        staking_type: StakingType,
        apr: u32,
        min_epochs_to_claim: u32,
        min_epochs_to_unstake: u32,
        min_epochs_to_withdraw: u32,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::UpdateStaking as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(staking_type as u32);
        arg_buffer.push_arg(apr);
        arg_buffer.push_arg(min_epochs_to_claim);
        arg_buffer.push_arg(min_epochs_to_unstake);
        arg_buffer.push_arg(min_epochs_to_withdraw);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous update fee pool of KDA. Execution is resumed afterwards.
    pub fn kda_update_fee_pool(
        &self,
        token: &TokenIdentifier<A>,
        is_active: bool,
        admin_address: &ManagedAddress<A>,
        f_ratio_kda: u64,
        f_ratio_klv: u64,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::UpdateKDAFeePool as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(is_active);
        arg_buffer.push_arg(admin_address);
        arg_buffer.push_arg(f_ratio_kda);
        arg_buffer.push_arg(f_ratio_klv);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous stop royalties change of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_stop_royalties_change(&self, token: &TokenIdentifier<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::StopRoyaltiesChange as u32);
        arg_buffer.push_arg(token);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous stop metadata change of NFT. Execution is resumed afterwards.
    pub fn kda_stop_metadata_change(&self, token: &TokenIdentifier<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(AssetTriggerType::StopNFTMetadataChange as u32);
        arg_buffer.push_arg(token);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous update royalties of KDA/NFT. Execution is resumed afterwards.
    pub fn kda_update_royalties(&self, token: &TokenIdentifier<A>, royalties: RoyaltiesData<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ASSET_TRIGGER_FUNC_NAME;

        let mut royalties_bytes = ManagedBuffer::<A>::new();
        let _ = royalties.dep_encode(&mut royalties_bytes);

        arg_buffer.push_arg(AssetTriggerType::UpdateRoyalties as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(royalties_bytes);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous KDA create. Execution is resumed afterwards.
    /// #[allow(clippy::too_many_arguments)]
    pub fn kda_create(
        &self,
        asset_type: AssetType,
        name: &ManagedBuffer<A>,
        ticker: &ManagedBuffer<A>,
        precision: u32,
        owner: &ManagedAddress<A>,
        logo: &ManagedBuffer<A>,
        initial_supply: &BigUint<A>,
        max_supply: &BigUint<A>,
        properties: &PropertiesInfo,
        royalties: &RoyaltiesData<A>,
    ) -> TokenIdentifier<A> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_CREATE_ASSET_FUNC_NAME;

        let mut royalties_bytes = ManagedBuffer::<A>::new();

        let _ = royalties.dep_encode(&mut royalties_bytes);

        arg_buffer.push_arg(asset_type as u32);
        arg_buffer.push_arg(name);
        arg_buffer.push_arg(ticker);
        arg_buffer.push_arg(precision);
        arg_buffer.push_arg(owner);
        arg_buffer.push_arg(logo);
        arg_buffer.push_arg(initial_supply);
        arg_buffer.push_arg(max_supply);
        arg_buffer.push_arg(properties.can_freeze);
        arg_buffer.push_arg(properties.can_wipe);
        arg_buffer.push_arg(properties.can_pause);
        arg_buffer.push_arg(properties.can_mint);
        arg_buffer.push_arg(properties.can_burn);
        arg_buffer.push_arg(properties.can_change_owner);
        arg_buffer.push_arg(properties.can_add_roles);
        arg_buffer.push_arg(properties.limit_transfer);
        arg_buffer.push_arg(royalties_bytes);

        let result = self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        );

        // try get index 0, if error signal error
        match result.try_get(0) {
            Some(result) => TokenIdentifier::from(result.clone_value()),
            None => A::error_api_impl().signal_error("KDA create failed".as_bytes()),
        }
    }

    /// Allows synchronous freeze of KDA. Execution is resumed afterwards.
    pub fn freeze(&self, token: &TokenIdentifier<A>, amount: &BigUint<A>) -> ManagedBuffer<A> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_FREEZE_FUNC_NAME;

        arg_buffer.push_arg(token);
        arg_buffer.push_arg(amount);

        let result = self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        );

        // try get index 0, if error signal error
        match result.try_get(0) {
            Some(result) => {
                // BUCKET ID
                result.clone_value()
            },
            None => A::error_api_impl().signal_error("Freeze failed".as_bytes()),
        }
    }

    /// Allows synchronous unfreeze of KDA. Execution is resumed afterwards.
    pub fn unfreeze(&self, token: &TokenIdentifier<A>, bucket_id: &ManagedBuffer<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_UNFREEZE_FUNC_NAME;

        arg_buffer.push_arg(token);
        arg_buffer.push_arg(bucket_id);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous delegate of KDA. Execution is resumed afterwards.
    pub fn delegate(&self, address: &ManagedAddress<A>, bucket_id: &ManagedBuffer<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_DELEGATE_FUNC_NAME;

        arg_buffer.push_arg(address);
        arg_buffer.push_arg(bucket_id);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous undelegate of KDA. Execution is resumed afterwards.
    pub fn undelegate(&self, bucket_id: &ManagedBuffer<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_UNDELEGATE_FUNC_NAME;

        arg_buffer.push_arg(bucket_id);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous claim of KDA. Execution is resumed afterwards.
    pub fn kda_claim(&self, claim_type: ClaimType, id: &ManagedBuffer<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_CLAIM_FUNC_NAME;

        arg_buffer.push_arg(claim_type as u32);
        arg_buffer.push_arg(id);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous staking claim of KDA. Execution is resumed afterwards.
    pub fn kda_claim_staking(&self, token: &TokenIdentifier<A>) {
        self.kda_claim(ClaimType::Staking, token.as_managed_buffer())
    }

    /// Allows synchronous allowance claim of KDA. Execution is resumed afterwards.
    pub fn kda_claim_allowance(&self, token: &TokenIdentifier<A>) {
        self.kda_claim(ClaimType::Allowance, token.as_managed_buffer())
    }

    /// Allows synchronous market claim of a market order. Execution is resumed afterwards.
    pub fn kda_claim_market_order(&self, order_id: &ManagedBuffer<A>) {
        self.kda_claim(ClaimType::Market, order_id)
    }

    /// Allows synchronous withdraw of KDA. Execution is resumed afterwards.
    pub fn withdraw(
        &self,
        withdraw_type: WithdrawType,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        currency: &TokenIdentifier<A>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_WITHDRAW_FUNC_NAME;

        arg_buffer.push_arg(withdraw_type as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(amount);
        arg_buffer.push_arg(currency);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous withdraw staking of KDA. Execution is resumed afterwards.
    pub fn withdraw_staking(&self, token: &TokenIdentifier<A>) {
        self.withdraw(
            WithdrawType::Staking,
            token,
            &BigUint::zero(),
            &TokenIdentifier::from_kda_bytes(&[0u8; 0]),
        )
    }

    /// Allows synchronous withdraw from pool of KDA. Execution is resumed afterwards.
    pub fn withdraw_from_kda_pool(
        &self,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        currency: &TokenIdentifier<A>,
    ) {
        self.withdraw(WithdrawType::KDAPool, token, amount, currency)
    }

    /// Allows synchronous sell NFT on a marketplace. Execution is resumed afterwards.
    pub fn sell(
        &self,
        sell_type: SellType,
        marketplace_id: &ManagedBuffer<A>,
        nft_id: &TokenIdentifier<A>,
        nft_nonce: u64,
        currency: &TokenIdentifier<A>,
        price: &BigUint<A>,
        reserve_price: &BigUint<A>,
        end_time: u64,
    ) -> ManagedBuffer<A> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_SELL_FUNC_NAME;

        arg_buffer.push_arg(sell_type as u32);
        arg_buffer.push_arg(marketplace_id);
        arg_buffer.push_arg(nft_id);
        arg_buffer.push_arg(nft_nonce);
        arg_buffer.push_arg(currency);
        arg_buffer.push_arg(price);
        arg_buffer.push_arg(reserve_price);
        arg_buffer.push_arg(end_time);

        let result = self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        );

        // try get index 0, if error signal error
        match result.try_get(0) {
            Some(result) => {
                // ORDER ID
                result.clone_value()
            },
            None => A::error_api_impl().signal_error("Sell failed".as_bytes()),
        }
    }

    /// Allows synchronous buy ITO/NFT. Execution is resumed afterwards.
    pub fn buy(
        &self,
        buy_type: BuyType,
        id: &ManagedBuffer<A>,
        currency: &TokenIdentifier<A>,
        amount: &BigUint<A>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_BUY_FUNC_NAME;

        arg_buffer.push_arg(buy_type as u32);
        arg_buffer.push_arg(id);
        arg_buffer.push_arg(currency);
        arg_buffer.push_arg(amount);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous buy ITO. Execution is resumed afterwards.
    pub fn buy_ito(
        &self,
        token: &TokenIdentifier<A>,
        currency: &TokenIdentifier<A>,
        amount: &BigUint<A>,
    ) {
        self.buy(BuyType::ITO, token.as_managed_buffer(), currency, amount)
    }

    /// Allows synchronous buy NFT. Execution is resumed afterwards.
    pub fn buy_nft(
        &self,
        order_id: &ManagedBuffer<A>,
        currency: &TokenIdentifier<A>,
        amount: &BigUint<A>,
    ) {
        self.buy(BuyType::Market, order_id, currency, amount)
    }

    /// Allows synchronous cancel NFT market order. Execution is resumed afterwards.
    pub fn cancel_market_order(&self, order_id: &ManagedBuffer<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_CANCEL_MARKET_ORDER_FUNC_NAME;

        arg_buffer.push_arg(order_id);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous create marketplace for NFT. Execution is resumed afterwards.
    pub fn create_marketplace(
        &self,
        name: &ManagedBuffer<A>,
        referral_address: &ManagedAddress<A>,
        referral_percentage: u32,
    ) -> ManagedBuffer<A> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_CREATE_MARKETPLACE_FUNC_NAME;

        arg_buffer.push_arg(name);
        arg_buffer.push_arg(referral_address);
        arg_buffer.push_arg(referral_percentage);

        let result = self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        );

        // try get index 0, if error signal error
        match result.try_get(0) {
            Some(result) => {
                // MARKETPLACE ID
                result.clone_value()
            },
            None => A::error_api_impl().signal_error("Create marketplace failed".as_bytes()),
        }
    }

    /// Allows synchronous config a marketplace. Execution is resumed afterwards.
    pub fn config_marketplace(
        &self,
        marketplace_id: &ManagedBuffer<A>,
        name: &ManagedBuffer<A>,
        referral_address: &ManagedAddress<A>,
        referral_percentage: u32,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_CONFIG_MARKETPLACE_FUNC_NAME;

        arg_buffer.push_arg(marketplace_id);
        arg_buffer.push_arg(name);
        arg_buffer.push_arg(referral_address);
        arg_buffer.push_arg(referral_percentage);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous deposit KDA. Execution is resumed afterwards.
    pub fn deposit(
        &self,
        deposit_type: DepositType,
        id: &ManagedBuffer<A>,
        currency: &TokenIdentifier<A>,
        amount: &BigUint<A>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_DEPOSIT_FUNC_NAME;

        arg_buffer.push_arg(deposit_type as u32);
        arg_buffer.push_arg(id);
        arg_buffer.push_arg(currency);
        arg_buffer.push_arg(amount);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous deposit KDA. Execution is resumed afterwards.
    pub fn deposit_fpr(
        &self,
        token: &TokenIdentifier<A>,
        currency: &TokenIdentifier<A>,
        amount: &BigUint<A>,
    ) {
        self.deposit(
            DepositType::FPR,
            token.as_managed_buffer(),
            currency,
            amount,
        )
    }

    /// Allows synchronous deposit KDA. Execution is resumed afterwards.
    pub fn deposit_kda_pool(
        &self,
        pool_id: &ManagedBuffer<A>,
        currency: &TokenIdentifier<A>,
        amount: &BigUint<A>,
    ) {
        self.deposit(DepositType::KDAPool, pool_id, currency, amount)
    }

    /// Allows synchronous vote proposal. Execution is resumed afterwards.
    pub fn vote(&self, proposal_id: u64, vote_type: VoteType, amount: &BigUint<A>) {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_VOTE_FUNC_NAME;

        arg_buffer.push_arg(proposal_id);
        arg_buffer.push_arg(vote_type as u32);
        arg_buffer.push_arg(amount);

        self.call_local_kda_built_in_function_minimal(func_name, arg_buffer);
    }

    /// Allows synchronous ITO config. Execution is resumed afterwards.
    /// #[allow(clippy::too_many_arguments)]
    pub fn ito_config(
        &self,
        token: &TokenIdentifier<A>,
        receiver: &ManagedAddress<A>,
        status: ITOStatus,
        max_amount: &BigUint<A>,
        default_limit_per_address: &BigUint<A>,
        start_time: u64,
        end_time: u64,
        whitelist_status: ITOWhitelistStatus,
        whitelist_start_time: u64,
        whitelist_end_time: u64,
        whitelist_info: &ManagedVec<A, ITOWhitelist<A>>,
        packs: &ManagedVec<A, ITOPackInfo<A>>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_CONFIG_ITO_FUNC_NAME;

        let mut whitelist_bytes = ManagedBuffer::<A>::new();
        let _ = whitelist_info.dep_encode(&mut whitelist_bytes);

        let mut packs_bytes = ManagedBuffer::<A>::new();
        let _ = packs.dep_encode(&mut packs_bytes);

        arg_buffer.push_arg(token);
        arg_buffer.push_arg(receiver);
        arg_buffer.push_arg(status as u32);
        arg_buffer.push_arg(max_amount);
        arg_buffer.push_arg(default_limit_per_address);
        arg_buffer.push_arg(start_time);
        arg_buffer.push_arg(end_time);
        arg_buffer.push_arg(whitelist_status as u32);
        arg_buffer.push_arg(whitelist_start_time);
        arg_buffer.push_arg(whitelist_end_time);
        arg_buffer.push_arg(whitelist_bytes);
        arg_buffer.push_arg(packs_bytes);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous set ITO prices. Execution is resumed afterwards.
    pub fn ito_set_prices(
        &self,
        token: &TokenIdentifier<A>,
        packs: &ManagedVec<A, ITOPackInfo<A>>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ITO_TRIGGER_FUNC_NAME;

        let mut packs_bytes = ManagedBuffer::<A>::new();
        let _ = packs.dep_encode(&mut packs_bytes);

        arg_buffer.push_arg(ITOTriggerType::SetITOPrices as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(packs_bytes);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous update ITO status. Execution is resumed afterwards.
    pub fn ito_update_status(
        &self,
        token: &TokenIdentifier<A>,
        status: ITOStatus,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ITO_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(ITOTriggerType::UpdateStatus as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(status as u32);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous update ITO receiver address. Execution is resumed afterwards.
    pub fn ito_update_receiver_address(
        &self,
        token: &TokenIdentifier<A>,
        receiver: &ManagedAddress<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ITO_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(ITOTriggerType::UpdateReceiverAddress as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(receiver);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous update ITO max amount. Execution is resumed afterwards.
    pub fn ito_update_max_amount(
        &self,
        token: &TokenIdentifier<A>,
        max_amount: &BigUint<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ITO_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(ITOTriggerType::UpdateMaxAmount as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(max_amount);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous update ITO default limit per address. Execution is resumed afterwards.
    pub fn ito_update_default_limit_per_address(
        &self,
        token: &TokenIdentifier<A>,
        default_limit_per_address: &BigUint<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ITO_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(ITOTriggerType::UpdateDefaultLimitPerAddress as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(default_limit_per_address);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous update ITO times. Execution is resumed afterwards.
    pub fn ito_update_times(
        &self,
        token: &TokenIdentifier<A>,
        start_time: u64,
        end_time: u64,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ITO_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(ITOTriggerType::UpdateTimes as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(start_time);
        arg_buffer.push_arg(end_time);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous update ITO whitelist status. Execution is resumed afterwards.
    pub fn ito_update_whitelist_status(
        &self,
        token: &TokenIdentifier<A>,
        whitelist_status: ITOWhitelistStatus,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ITO_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(ITOTriggerType::UpdateWhitelistStatus as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(whitelist_status as u32);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous add addresses to ITO whitelist. Execution is resumed afterwards.
    pub fn ito_add_to_whitelist(
        &self,
        token: &TokenIdentifier<A>,
        whitelist_info: &ManagedVec<A, ITOWhitelist<A>>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ITO_TRIGGER_FUNC_NAME;

        let mut whitelist_bytes = ManagedBuffer::<A>::new();
        let _ = whitelist_info.dep_encode(&mut whitelist_bytes);

        arg_buffer.push_arg(ITOTriggerType::AddToWhitelist as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(whitelist_bytes);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous remove addresses from ITO whitelist. Execution is resumed afterwards.
    pub fn ito_remove_from_whitelist(
        &self,
        token: &TokenIdentifier<A>,
        whitelist_info: &ManagedVec<A, ITOWhitelist<A>>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ITO_TRIGGER_FUNC_NAME;

        let mut whitelist_bytes = ManagedBuffer::<A>::new();
        let _ = whitelist_info.dep_encode(&mut whitelist_bytes);

        arg_buffer.push_arg(ITOTriggerType::RemoveFromWhitelist as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(whitelist_bytes);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous update ITO whitelist times. Execution is resumed afterwards.
    pub fn ito_update_whitelist_times(
        &self,
        token: &TokenIdentifier<A>,
        whitelist_start_time: u64,
        whitelist_end_time: u64,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_ITO_TRIGGER_FUNC_NAME;

        arg_buffer.push_arg(ITOTriggerType::UpdateWhitelistTimes as u32);
        arg_buffer.push_arg(token);
        arg_buffer.push_arg(whitelist_start_time);
        arg_buffer.push_arg(whitelist_end_time);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }

    /// Allows synchronous set an account name. Execution is resumed afterwards.
    pub fn set_account_name(&self, name: ManagedBuffer<A>) -> ManagedVec<A, ManagedBuffer<A>> {
        let mut arg_buffer = ManagedArgBuffer::new();
        let func_name = KLEVER_SET_ACCOUNT_NAME_FUNC_NAME;

        arg_buffer.push_arg(name);

        self.call_kda_built_in_function(
            A::blockchain_api_impl().get_gas_left(),
            ManagedBuffer::from(func_name),
            arg_buffer
        )
    }
}
