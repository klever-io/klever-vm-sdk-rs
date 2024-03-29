use crate::api::VmApiImpl;
use klever_sc::api::{const_handles, RawHandle, SendApi, SendApiImpl};

extern "C" {
    fn managedMultiTransferKDANFTExecute(
        dstHandle: i32,
        tokenTransfersHandle: i32,
        gasLimit: i64,
        functionHandle: i32,
        argumentsHandle: i32,
    ) -> i32;

    fn managedExecuteOnDestContext(
        gas: i64,
        addressHandle: i32,
        valueHandle: i32,
        functionHandle: i32,
        argumentsHandle: i32,
        resultHandle: i32,
    ) -> i32;

    fn managedExecuteOnSameContext(
        gas: i64,
        addressHandle: i32,
        valueHandle: i32,
        functionHandle: i32,
        argumentsHandle: i32,
        resultHandle: i32,
    ) -> i32;

    fn managedExecuteReadOnly(
        gas: i64,
        addressHandle: i32,
        functionHandle: i32,
        argumentsHandle: i32,
        resultHandle: i32,
    ) -> i32;

    fn managedCreateContract(
        gas: i64,
        valueHandle: i32,
        codeHandle: i32,
        codeMetadataHandle: i32,
        argumentsHandle: i32,
        resultAddressHandle: i32,
        resultHandle: i32,
    ) -> i32;

    fn managedDeployFromSourceContract(
        gas: i64,
        valueHandle: i32,
        addressHandle: i32,
        codeMetadataHandle: i32,
        argumentsHandle: i32,
        resultAddressHandle: i32,
        resultHandle: i32,
    ) -> i32;

    fn managedUpgradeContract(
        dstHandle: i32,
        gas: i64,
        valueHandle: i32,
        codeHandle: i32,
        codeMetadataHandle: i32,
        argumentsHandle: i32,
        resultHandle: i32,
    );

    fn managedUpgradeFromSourceContract(
        dstHandle: i32,
        gas: i64,
        valueHandle: i32,
        addressHandle: i32,
        codeMetadataHandle: i32,
        argumentsHandle: i32,
        resultHandle: i32,
    );

    // #[allow(unused)]
    // fn managedGetReturnData(resultID: i32, resultHandle: i32);

    /// Clears results propagated from nested sync calls
    fn cleanReturnData();
    fn deleteFromReturnData(resultID: i32);
}

impl SendApi for VmApiImpl {
    type SendApiImpl = VmApiImpl;

    #[inline]
    fn send_api_impl() -> Self::SendApiImpl {
        VmApiImpl {}
    }
}

impl SendApiImpl for VmApiImpl {
    fn multi_transfer_kda_nft_execute(
        &self,
        to_handle: RawHandle,
        payments_handle: RawHandle,
        gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let result = managedMultiTransferKDANFTExecute(
                to_handle,
                payments_handle,
                gas_limit as i64,
                endpoint_name_handle,
                arg_buffer_handle,
            );
            if result == 0 {
                Ok(())
            } else {
                Err(b"multiTransferKDANFTExecute failed")
            }
        }
    }

    fn deploy_contract(
        &self,
        gas: u64,
        klv_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        unsafe {
            let _ = managedCreateContract(
                gas as i64,
                klv_value_handle,
                code_handle,
                code_metadata_handle,
                arg_buffer_handle,
                new_address_handle,
                result_handle,
            );
        }
    }

    fn deploy_from_source_contract(
        &self,
        gas: u64,
        klv_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        unsafe {
            let _ = managedDeployFromSourceContract(
                gas as i64,
                klv_value_handle,
                source_contract_address_handle,
                code_metadata_handle,
                arg_buffer_handle,
                new_address_handle,
                result_handle,
            );
        }
    }

    fn upgrade_from_source_contract(
        &self,
        sc_address_handle: RawHandle,
        gas: u64,
        klv_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) {
        unsafe {
            let unused_result_handle = const_handles::MBUF_TEMPORARY_1;
            managedUpgradeFromSourceContract(
                sc_address_handle,
                gas as i64,
                klv_value_handle,
                source_contract_address_handle,
                code_metadata_handle,
                arg_buffer_handle,
                unused_result_handle,
            );
        }
    }

    fn upgrade_contract(
        &self,
        sc_address_handle: RawHandle,
        gas: u64,
        klv_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) {
        unsafe {
            let unused_result_handle = const_handles::MBUF_TEMPORARY_1;
            managedUpgradeContract(
                sc_address_handle,
                gas as i64,
                klv_value_handle,
                code_handle,
                code_metadata_handle,
                arg_buffer_handle,
                unused_result_handle,
            );

            // Note: the result handle is a mistake in the EI.
            // The upgrade contract operation is an async call, so no results can be returned.
        }
    }

    fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        to_handle: RawHandle,
        klv_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        unsafe {
            let _ = managedExecuteOnDestContext(
                gas as i64,
                to_handle,
                klv_value_handle,
                endpoint_name_handle,
                arg_buffer_handle,
                result_handle,
            );
        }
    }

    fn execute_on_same_context_raw(
        &self,
        gas: u64,
        to_handle: RawHandle,
        klv_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        unsafe {
            let _ = managedExecuteOnSameContext(
                gas as i64,
                to_handle,
                klv_value_handle,
                endpoint_name_handle,
                arg_buffer_handle,
                result_handle,
            );
        }
    }

    fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        to_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        unsafe {
            let _ = managedExecuteReadOnly(
                gas as i64,
                to_handle,
                endpoint_name_handle,
                arg_buffer_handle,
                result_handle,
            );
        }
    }

    fn clean_return_data(&self) {
        unsafe {
            cleanReturnData();
        }
    }

    fn delete_from_return_data(&self, index: usize) {
        unsafe {
            deleteFromReturnData(index as i32);
        }
    }
}
