use klever_sc::api::{const_handles, RawHandle};

#[derive(Debug)]
pub struct TxStaticVars {
    pub external_view_target_address_handle: RawHandle,
    pub next_handle: RawHandle,
    pub num_arguments: i32,
    pub call_value_klv_handle: RawHandle,
    pub call_value_multi_kda_handle: RawHandle,
    pub call_value_multi_kda_no_klv_handle: RawHandle,
}

impl Default for TxStaticVars {
    fn default() -> Self {
        TxStaticVars {
            external_view_target_address_handle: 0,
            next_handle: const_handles::NEW_HANDLE_START_FROM,
            num_arguments: -1,
            call_value_klv_handle: const_handles::UNINITIALIZED_HANDLE,
            call_value_multi_kda_handle: const_handles::UNINITIALIZED_HANDLE,
            call_value_multi_kda_no_klv_handle: const_handles::UNINITIALIZED_HANDLE,
        }
    }
}
