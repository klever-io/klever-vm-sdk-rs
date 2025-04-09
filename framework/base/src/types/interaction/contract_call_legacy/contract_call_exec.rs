use crate::types::{decode_result, TRANSFER_EXECUTE_DEFAULT_LEFTOVER, UNSPECIFIED_GAS_LIMIT};
use crate::{
    api::{use_raw_handle, HandleConstraints, StaticVarApiImpl},
    codec::TopDecodeMulti,
};
use crate::{
    api::{BlockchainApiImpl, CallTypeApi},
    contract_base::SendRawWrapper,
    formatter::SCLowerHex,
    types::{
        BigUint, KdaTokenPayment, ManagedBuffer, ManagedBufferBuilder, ManagedType, ManagedVec,
    },
};

use super::{ContractCallNoPayment, ContractCallWithKlv};

impl<SA, OriginalResult> ContractCallWithKlv<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn resolve_gas_limit(&self) -> u64 {
        if self.basic.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            SA::blockchain_api_impl().get_gas_left()
        } else {
            self.basic.explicit_gas_limit
        }
    }

    #[inline]
    pub fn get_back_transfers(&self) -> (BigUint<SA>, ManagedVec<SA, KdaTokenPayment<SA>>) {
        let kda_transfer_value_handle: SA::BigIntHandle =
            use_raw_handle(SA::static_var_api_impl().next_handle());
        let call_value_handle: SA::BigIntHandle =
            use_raw_handle(SA::static_var_api_impl().next_handle());

        SA::blockchain_api_impl().managed_get_back_transfers(
            kda_transfer_value_handle.get_raw_handle(),
            call_value_handle.get_raw_handle(),
        );

        (
            BigUint::from_raw_handle(call_value_handle.get_raw_handle()),
            ManagedVec::from_raw_handle(kda_transfer_value_handle.get_raw_handle()),
        )
    }

    pub fn to_call_data_string(&self) -> ManagedBuffer<SA> {
        let mut result = ManagedBufferBuilder::default();
        result.append_managed_buffer(&self.basic.function_call.function_name);
        for arg in self.basic.function_call.arg_buffer.raw_arg_iter() {
            result.append_bytes(b"@");
            SCLowerHex::fmt(&*arg, &mut result);
        }
        result.into_managed_buffer()
    }

    /// Executes immediately, synchronously, and returns contract call result.
    /// Only works if the target contract is in the same shard.
    pub(super) fn execute_on_dest_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let raw_result = SendRawWrapper::<SA>::new().execute_on_dest_context_raw(
            self.resolve_gas_limit(),
            &self.basic.to,
            &self.klv_payment,
            &self.basic.function_call.function_name,
            &self.basic.function_call.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }

    pub(super) fn execute_on_dest_context_readonly<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let raw_result = SendRawWrapper::<SA>::new().execute_on_dest_context_readonly_raw(
            self.resolve_gas_limit(),
            &self.basic.to,
            &self.basic.function_call.function_name,
            &self.basic.function_call.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }

    pub(super) fn execute_on_same_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let raw_result = SendRawWrapper::<SA>::new().execute_on_same_context_raw(
            self.resolve_gas_limit(),
            &self.basic.to,
            &self.klv_payment,
            &self.basic.function_call.function_name,
            &self.basic.function_call.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }
}

impl<SA, OriginalResult> ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) fn resolve_gas_limit_with_leftover(&self) -> u64 {
        if self.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            let mut gas_left = SA::blockchain_api_impl().get_gas_left();
            if gas_left > TRANSFER_EXECUTE_DEFAULT_LEFTOVER {
                gas_left -= TRANSFER_EXECUTE_DEFAULT_LEFTOVER;
            }
            gas_left
        } else {
            self.explicit_gas_limit
        }
    }

    pub(super) fn transfer_execute_multi_kda(self, payments: ManagedVec<SA, KdaTokenPayment<SA>>) {
        let gas_limit = self.resolve_gas_limit_with_leftover();
        _ = SendRawWrapper::<SA>::new().multi_kda_transfer_execute(
            &self.to,
            &payments,
            gas_limit,
            &self.function_call.function_name,
            &self.function_call.arg_buffer,
        );
    }

    pub(super) fn transfer_execute_kda(self, payments: ManagedVec<SA, KdaTokenPayment<SA>>) {
        self.transfer_execute_multi_kda(payments);
    }
}
