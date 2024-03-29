use std::ops::Add;

use crate::{types::RawHandle, vm_err_msg, vm_hooks::VMHooksHandlerSource};
use num_traits::Zero;

use super::VMHooksManagedTypes;

pub trait VMHooksCallValue: VMHooksHandlerSource + VMHooksManagedTypes {
    fn check_not_payable(&self) {
        if self.input_ref().klv_value > num_bigint::BigUint::zero() {
            self.vm_error(vm_err_msg::NON_PAYABLE_FUNC_KLV);
        }
        if self.kda_num_transfers() > 0 {
            self.vm_error(vm_err_msg::NON_PAYABLE_FUNC_KDA);
        }
    }

    fn load_klv_value(&self, dest: RawHandle) {
        let value = self.input_ref().received_klv().clone();
        self.m_types_lock().bi_overwrite(dest, value.into());
    }

    fn load_kda_value(&self, dest: RawHandle, token_id_handle: RawHandle) {
        let transfers = self.input_ref().received_kda();
        let token_id_bytes = self.m_types_lock().mb_get(token_id_handle).to_vec();
        // search for the token id
        let mut value = num_bigint::BigUint::zero();
        for transfer in transfers {
            if transfer.token_identifier == token_id_bytes {
                value = value.add(transfer.value.clone());
            }
        }

        self.m_types_lock().bi_overwrite(dest, value.into());
    }

    fn load_all_kda_transfers(&self, dest_handle: RawHandle) {
        let transfers = self.input_ref().received_kda();
        self.m_types_lock()
            .mb_set_vec_of_kda_payments(dest_handle, transfers);
    }

    fn kda_num_transfers(&self) -> usize {
        self.input_ref().received_kda().len()
    }
}
