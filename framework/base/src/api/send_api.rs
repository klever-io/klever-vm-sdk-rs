use super::{BlockchainApi, HandleTypeInfo, ManagedTypeApi, ManagedTypeApiImpl, RawHandle};

pub trait SendApi: ManagedTypeApi + BlockchainApi {
    type SendApiImpl: SendApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn send_api_impl() -> Self::SendApiImpl;
}

/// API that groups methods that either send KLV or KDA, or that call other contracts.
pub trait SendApiImpl: ManagedTypeApiImpl {
    fn multi_transfer_kda_nft_execute(
        &self,
        to_handle: RawHandle,
        payments_handle: RawHandle,
        gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), &'static [u8]>;

    /// Deploys a new contract.
    /// the deployment is synchronous and tx execution continues afterwards.
    /// it uses an argument buffer to pass arguments
    /// If the deployment fails, Option::None is returned
    #[allow(clippy::too_many_arguments)]
    fn deploy_contract(
        &self,
        gas: u64,
        klv_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    );

    /// Deploys a new contract by re-using the code of an already deployed source contract.
    /// The deployment is done synchronously and the new contract's address is returned.
    /// If the deployment fails, Option::None is returned
    #[allow(clippy::too_many_arguments)]
    fn deploy_from_source_contract(
        &self,
        gas: u64,
        klv_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    );

    fn upgrade_from_source_contract(
        &self,
        sc_address_handle: RawHandle,
        gas: u64,
        klv_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    );

    /// Upgrades a child contract of the currently executing contract.
    /// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
    /// The child contract's new init function will be called with the provided arguments
    fn upgrade_contract(
        &self,
        sc_address_handle: RawHandle,
        gas: u64,
        klv_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    );

    /// In-line execution of another contract.
    fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address_handle: RawHandle,
        klv_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    );

    fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address_handle: RawHandle,
        klv_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    );

    fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        address_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    );

    fn clean_return_data(&self);

    fn delete_from_return_data(&self, index: usize);
}
