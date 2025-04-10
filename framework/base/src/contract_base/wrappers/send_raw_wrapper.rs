use core::marker::PhantomData;

use crate::{
    api::{
        const_handles, use_raw_handle, BigIntApiImpl, BlockchainApiImpl, CallTypeApi,
        HandleConstraints, ManagedBufferApiImpl, RawHandle, SendApiImpl, StaticVarApiImpl,
    },
    types::{
        BigUint, CodeMetadata, KdaTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
        ManagedType, ManagedVec, TokenIdentifier,
    },
};

#[derive(Default)]
pub struct SendRawWrapper<A>
where
    A: CallTypeApi,
{
    _phantom: PhantomData<A>,
}

impl<A> SendRawWrapper<A>
where
    A: CallTypeApi,
{
    pub fn new() -> Self {
        SendRawWrapper {
            _phantom: PhantomData,
        }
    }

    fn load_code_metadata_to_mb(
        &self,
        code_metadata: CodeMetadata,
        code_metadata_handle: RawHandle,
    ) {
        let code_metadata_bytes = code_metadata.to_byte_array();
        A::managed_type_impl().mb_overwrite(
            use_raw_handle(code_metadata_handle),
            &code_metadata_bytes[..],
        );
    }

    pub fn transfer_kda_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        value: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        self.transfer_kda_nft_execute(to, token, 0, value, gas_limit, endpoint_name, arg_buffer)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn transfer_kda_nft_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        nonce: u64,
        value: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        let mut payments: ManagedVec<A, KdaTokenPayment<A>> = ManagedVec::new();
        payments.push(KdaTokenPayment::new(token.clone(), nonce, value.clone()));
        self.multi_kda_transfer_execute(to, &payments, gas_limit, endpoint_name, arg_buffer)
    }

    pub fn multi_kda_transfer_execute(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, KdaTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().multi_transfer_kda_nft_execute(
            to.get_handle().get_raw_handle(),
            payments.get_handle().get_raw_handle(),
            gas_limit,
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
        )
    }
    /// Deploys a new contract in the same shard.
    /// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
    /// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
    /// If the deployment fails, Option::None is returned
    pub fn deploy_contract(
        &self,
        gas: u64,
        klv_value: &BigUint<A>,
        code: &ManagedBuffer<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> (ManagedAddress<A>, ManagedVec<A, ManagedBuffer<A>>) {
        let code_metadata_handle = const_handles::MBUF_TEMPORARY_1;
        self.load_code_metadata_to_mb(code_metadata, code_metadata_handle);
        let new_address_handle = A::static_var_api_impl().next_handle();
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().deploy_contract(
            gas,
            klv_value.get_handle().get_raw_handle(),
            code.get_handle().get_raw_handle(),
            code_metadata_handle,
            arg_buffer.get_handle().get_raw_handle(),
            new_address_handle,
            result_handle,
        );
        (
            ManagedAddress::from_raw_handle(new_address_handle),
            ManagedVec::from_raw_handle(result_handle),
        )
    }

    /// Deploys a new contract in the same shard by re-using the code of an already deployed source contract.
    /// The deployment is done synchronously and the new contract's address is returned.
    /// If the deployment fails, Option::None is returned
    pub fn deploy_from_source_contract(
        &self,
        gas: u64,
        klv_value: &BigUint<A>,
        source_contract_address: &ManagedAddress<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> (ManagedAddress<A>, ManagedVec<A, ManagedBuffer<A>>) {
        let code_metadata_handle = const_handles::MBUF_TEMPORARY_1;
        self.load_code_metadata_to_mb(code_metadata, code_metadata_handle);
        let new_address_handle = A::static_var_api_impl().next_handle();
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().deploy_from_source_contract(
            gas,
            klv_value.get_handle().get_raw_handle(),
            source_contract_address.get_handle().get_raw_handle(),
            code_metadata_handle,
            arg_buffer.get_handle().get_raw_handle(),
            new_address_handle,
            result_handle,
        );
        (
            ManagedAddress::from_raw_handle(new_address_handle),
            ManagedVec::from_raw_handle(result_handle),
        )
    }

    pub fn upgrade_from_source_contract(
        &self,
        sc_address: &ManagedAddress<A>,
        gas: u64,
        klv_value: &BigUint<A>,
        source_contract_address: &ManagedAddress<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let code_metadata_handle = const_handles::MBUF_TEMPORARY_1;
        self.load_code_metadata_to_mb(code_metadata, code_metadata_handle);
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().upgrade_from_source_contract(
            sc_address.get_handle().get_raw_handle(),
            gas,
            klv_value.get_handle().get_raw_handle(),
            source_contract_address.get_handle().get_raw_handle(),
            code_metadata_handle,
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );

        ManagedVec::from_raw_handle(result_handle)
    }

    /// Upgrades a child contract of the currently executing contract.
    /// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
    /// The child contract's new init function will be called with the provided arguments
    pub fn upgrade_contract(
        &self,
        sc_address: &ManagedAddress<A>,
        gas: u64,
        klv_value: &BigUint<A>,
        code: &ManagedBuffer<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let code_metadata_handle = const_handles::MBUF_TEMPORARY_1;
        self.load_code_metadata_to_mb(code_metadata, code_metadata_handle);
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().upgrade_contract(
            sc_address.get_handle().get_raw_handle(),
            gas,
            klv_value.get_handle().get_raw_handle(),
            code.get_handle().get_raw_handle(),
            code_metadata_handle,
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );

        ManagedVec::from_raw_handle(result_handle)
    }

    /// Deletes a contract.
    pub fn delete_contract(
        &self,
        address: &ManagedAddress<A>,
        gas: u64,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        A::send_api_impl().delete_contract(
            address.get_handle().get_raw_handle(),
            gas,
            arg_buffer.get_handle().get_raw_handle(),
        );
    }

    /// Same shard, in-line execution of another contract.
    pub fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().execute_on_dest_context_raw(
            gas,
            address.get_handle().get_raw_handle(),
            value.get_handle().get_raw_handle(),
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );
        ManagedVec::from_raw_handle(result_handle)
    }

    pub fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().execute_on_same_context_raw(
            gas,
            address.get_handle().get_raw_handle(),
            value.get_handle().get_raw_handle(),
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );
        ManagedVec::from_raw_handle(result_handle)
    }

    /// Same shard, in-line execution of another contract.
    pub fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().execute_on_dest_context_readonly_raw(
            gas,
            address.get_handle().get_raw_handle(),
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );
        ManagedVec::from_raw_handle(result_handle)
    }

    // TODO: add call/issue costs
    /// Allows synchronously calling a local function by name. Execution is resumed afterwards.
    pub fn call_kda_built_in_function(
        &self,
        gas: u64,
        function_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        // account-level built-in function, so the destination address is the contract itself
        let own_address_handle: A::ManagedBufferHandle =
            use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        // write the SC address to own_address_handle
        A::blockchain_api_impl().load_sc_address_managed(own_address_handle.clone());
        let klv_value_handle = A::managed_type_impl().bi_new_zero();

        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().execute_on_dest_context_raw(
            gas,
            own_address_handle.get_raw_handle(),
            klv_value_handle.get_raw_handle(),
            function_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );

        self.clean_return_data();

        ManagedVec::from_raw_handle(result_handle)
    }

    pub fn clean_return_data(&self) {
        A::send_api_impl().clean_return_data()
    }
}
