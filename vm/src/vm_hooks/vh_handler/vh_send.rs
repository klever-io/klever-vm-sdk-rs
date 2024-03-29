use crate::{
    tx_execution::builtin_function_names::{KLEVER_TRANSFER_FUNC_NAME, UPGRADE_CONTRACT_FUNC_NAME},
    tx_mock::{TxFunctionName, TxTokenTransfer}, 
    types::{top_encode_big_uint, top_encode_u64, RawHandle, VMAddress, VMCodeMetadata},
    vm_hooks::VMHooksHandlerSource
};
use num_traits::Zero;

fn append_endpoint_name_and_args(
    args: &mut Vec<Vec<u8>>,
    endpoint_name: TxFunctionName,
    arg_buffer: Vec<Vec<u8>>,
) {
    if !endpoint_name.is_empty() {
        args.push(endpoint_name.into_bytes());
        args.extend(arg_buffer);
    }
}

pub trait VMHooksSend: VMHooksHandlerSource {
    fn perform_transfer_execute_multi(
        &self,
        to: VMAddress,
        payments: Vec<TxTokenTransfer>,
        _gas_limit: u64,
        endpoint_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) {
        let contract_address = self.current_address().clone();

        let mut args = vec![to.to_vec(), top_encode_u64(payments.len() as u64)];

        for payment in payments.into_iter() {
            let token_bytes = payment.token_identifier;
            args.push(token_bytes);
            let nonce_bytes = top_encode_u64(payment.nonce);
            args.push(nonce_bytes);
            let amount_bytes = top_encode_big_uint(&payment.value);
            args.push(amount_bytes);
        }

        append_endpoint_name_and_args(&mut args, endpoint_name, arguments);

        self.perform_transfer_execute(
            contract_address,
            num_bigint::BigUint::zero(),
            KLEVER_TRANSFER_FUNC_NAME.into(),
            args,
        );
    }

    fn perform_upgrade_contract(
        &self,
        to: VMAddress,
        klv_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        code_metadata: VMCodeMetadata,
        args: Vec<Vec<u8>>,
    ) {
        let mut arguments = vec![contract_code, code_metadata.to_vec()];
        arguments.extend(args);
        self.perform_execute_on_dest_context(to, klv_value, UPGRADE_CONTRACT_FUNC_NAME.into(), arguments);
    }

    fn multi_transfer_kda_nft_execute(
        &self,
        to_handle: RawHandle,
        payments_handle: RawHandle,
        gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) {
        let to = self.m_types_lock().mb_to_address(to_handle);
        let payments = self
            .m_types_lock()
            .mb_get_vec_of_kda_payments(payments_handle);
        let endpoint_name = self
            .m_types_lock()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.m_types_lock().mb_get_vec_of_bytes(arg_buffer_handle);

        self.perform_transfer_execute_multi(to, payments, gas_limit, endpoint_name, arg_buffer)
    }


    #[allow(clippy::too_many_arguments)]
    fn deploy_contract(
        &self,
        _gas: u64,
        klv_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        let klv_value = self.m_types_lock().bu_get(klv_value_handle);
        let code = self.m_types_lock().mb_get(code_handle).to_vec();
        let code_metadata = self
            .m_types_lock()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.m_types_lock().mb_get_vec_of_bytes(arg_buffer_handle);

        let (new_address, result) =
            self.perform_deploy(klv_value, code, code_metadata, arg_buffer);

        self.m_types_lock()
            .mb_set(new_address_handle, new_address.to_vec());
        self.m_types_lock()
            .mb_set_vec_of_bytes(result_handle, result);
    }

    #[allow(clippy::too_many_arguments)]
    fn deploy_from_source_contract(
        &self,
        _gas: u64,
        klv_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        let klv_value = self.m_types_lock().bu_get(klv_value_handle);
        let source_contract_address = self
            .m_types_lock()
            .mb_to_address(source_contract_address_handle);
        let source_contract_code = self.account_code(&source_contract_address);
        let code_metadata = self
            .m_types_lock()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.m_types_lock().mb_get_vec_of_bytes(arg_buffer_handle);

        let (new_address, result) =
            self.perform_deploy(klv_value, source_contract_code, code_metadata, arg_buffer);

        self.m_types_lock()
            .mb_set(new_address_handle, new_address.to_vec());
        self.m_types_lock()
            .mb_set_vec_of_bytes(result_handle, result);
    }

    fn upgrade_from_source_contract(
        &self,
        sc_address_handle: RawHandle,
        _gas: u64,
        klv_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) {
        let to = self.m_types_lock().mb_to_address(sc_address_handle);
        let klv_value = self.m_types_lock().bu_get(klv_value_handle);
        let source_contract_address = self
            .m_types_lock()
            .mb_to_address(source_contract_address_handle);
        let source_contract_code = self.account_code(&source_contract_address);
        let code_metadata = self
            .m_types_lock()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.m_types_lock().mb_get_vec_of_bytes(arg_buffer_handle);

        self.perform_upgrade_contract(
            to,
            klv_value,
            source_contract_code,
            code_metadata,
            arg_buffer,
        )
    }

    fn upgrade_contract(
        &self,
        sc_address_handle: RawHandle,
        _gas: u64,
        klv_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) {
        let to = self.m_types_lock().mb_to_address(sc_address_handle);
        let klv_value = self.m_types_lock().bu_get(klv_value_handle);
        let code = self.m_types_lock().mb_get(code_handle).to_vec();
        let code_metadata = self
            .m_types_lock()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.m_types_lock().mb_get_vec_of_bytes(arg_buffer_handle);

        self.perform_upgrade_contract(to, klv_value, code, code_metadata, arg_buffer)
    }

    fn execute_on_dest_context_raw(
        &self,
        _gas: u64,
        to_handle: RawHandle,
        klv_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        let to = self.m_types_lock().mb_to_address(to_handle);
        let klv_value = self.m_types_lock().bu_get(klv_value_handle);
        let endpoint_name = self
            .m_types_lock()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.m_types_lock().mb_get_vec_of_bytes(arg_buffer_handle);

        let result =
            self.perform_execute_on_dest_context(to, klv_value, endpoint_name, arg_buffer);

        self.m_types_lock()
            .mb_set_vec_of_bytes(result_handle, result);
    }

    fn clean_return_data(&self) {
        let mut tx_result = self.result_lock();
        tx_result.result_values.clear();
    }

    fn delete_from_return_data(&self, index: usize) {
        let mut tx_result = self.result_lock();
        if index > tx_result.result_values.len() {
            return;
        }

        let _ = tx_result.result_values.remove(index);
    }
}
