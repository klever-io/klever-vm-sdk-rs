use crate::api::{RawHandle, SendApi, SendApiImpl};

use super::UncallableApi;

impl SendApi for UncallableApi {
    type SendApiImpl = UncallableApi;

    fn send_api_impl() -> Self::SendApiImpl {
        unreachable!()
    }
}

impl SendApiImpl for UncallableApi {
    fn multi_transfer_kda_nft_execute(
        &self,
        _to_handle: RawHandle,
        _payments_handle: RawHandle,
        _gas_limit: u64,
        _endpoint_name_handle: RawHandle,
        _arg_buffer_handle: RawHandle,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn deploy_contract(
        &self,
        _gas: u64,
        _amount_handle: RawHandle,
        _code_handle: RawHandle,
        _code_metadata_handle: RawHandle,
        _arg_buffer_handle: RawHandle,
        _new_address_handle: RawHandle,
        _result_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn deploy_from_source_contract(
        &self,
        _gas: u64,
        _amount_handle: RawHandle,
        _source_contract_address_handle: RawHandle,
        _code_metadata_handle: RawHandle,
        _arg_buffer_handle: RawHandle,
        _new_address_handle: RawHandle,
        _result_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn upgrade_from_source_contract(
        &self,
        _sc_address: RawHandle,
        _gas: u64,
        _amount_handle: RawHandle,
        _source_contract_address_handle: RawHandle,
        _code_metadata_handle: RawHandle,
        _arg_buffer_handle: RawHandle,
        _result_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn upgrade_contract(
        &self,
        _sc_address: RawHandle,
        _gas: u64,
        _amount_handle: RawHandle,
        _code_handle: RawHandle,
        _code_metadata_handle: RawHandle,
        _arg_buffer_handle: RawHandle,
        _result_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn delete_contract(
        &self,
        _address_handle: RawHandle,
        _gas: u64,
        _arg_buffer_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn execute_on_dest_context_raw(
        &self,
        _gas: u64,
        _address: RawHandle,
        _value: RawHandle,
        _endpoint_name_handle: RawHandle,
        _arg_buffer_handle: RawHandle,
        _result_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn execute_on_same_context_raw(
        &self,
        _gas: u64,
        _address: RawHandle,
        _value: RawHandle,
        _endpoint_name_handle: RawHandle,
        _arg_buffer_handle: RawHandle,
        _result_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn execute_on_dest_context_readonly_raw(
        &self,
        _gas: u64,
        _address: RawHandle,
        _endpoint_name_handle: RawHandle,
        _arg_buffer_handle: RawHandle,
        _result_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn clean_return_data(&self) {
        unreachable!()
    }

    fn delete_from_return_data(&self, _index: usize) {
        unreachable!()
    }
}
