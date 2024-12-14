use super::VmApiImpl;
use klever_sc::api::{CallValueApi, CallValueApiImpl};

extern "C" {
    fn checkNoPayment();

    fn bigIntGetCallValue(dest: i32);

    fn managedGetKDACallValue(dest: i32, token_id_handle: i32);

    fn managedGetMultiKDACallValue(resultHandle: i32);

    fn managedGetMultiKDAWithoutKLVCallValue(resultHandle: i32);

    fn getNumKDATransfers() -> i32;
}

impl CallValueApi for VmApiImpl {
    type CallValueApiImpl = VmApiImpl;

    #[inline]
    fn call_value_api_impl() -> Self::CallValueApiImpl {
        VmApiImpl {}
    }
}

impl CallValueApiImpl for VmApiImpl {
    #[inline]
    fn check_not_payable(&self) {
        unsafe {
            checkNoPayment();
        }
    }

    fn load_klv_value(&self, dest: Self::BigIntHandle) {
        unsafe {
            bigIntGetCallValue(dest);
        }
    }

    fn load_kda_value(&self, dest: Self::BigIntHandle, token_id_handle: Self::ManagedBufferHandle) {
        unsafe {
            managedGetKDACallValue(dest, token_id_handle);
        }
    }

    fn load_all_kda_transfers(&self, dest_handle: Self::ManagedBufferHandle) {
        unsafe {
            managedGetMultiKDACallValue(dest_handle);
        }
    }

    fn load_all_kda_transfers_no_klv(&self, dest_handle: Self::ManagedBufferHandle) {
        unsafe {
            managedGetMultiKDAWithoutKLVCallValue(dest_handle);
        }
    }

    fn kda_num_transfers(&self) -> usize {
        unsafe { getNumKDATransfers() as usize }
    }
}
