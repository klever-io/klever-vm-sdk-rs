use core::marker::PhantomData;

use crate::{
    api::{
        CallTypeApi, KLEVER_TRANSFER_FUNC_NAME,
    },
    types::{BigUint, KdaTokenPayment, ManagedVec},
};

use super::{
    contract_call_no_payment::ContractCallNoPayment, ContractCallWithKlv, ManagedArgBuffer,
};

impl<SA, OriginalResult> ContractCallWithKlv<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// If this is a KDA call, it converts it to a regular call to KleverTransfer.
    pub fn convert_to_kda_transfer_call(
        self,
        payments: ManagedVec<SA, KdaTokenPayment<SA>>,
    ) -> Self {
        match payments.len() {
            0 => self,
            _ => self.convert_to_multi_transfer_kda_call(payments),
        }
    }

    fn convert_to_multi_transfer_kda_call(
        self,
        payments: ManagedVec<SA, KdaTokenPayment<SA>>,
    ) -> Self {
        let mut new_arg_buffer = ManagedArgBuffer::new();
        new_arg_buffer.push_arg(self.basic.to.clone());
        new_arg_buffer.push_arg(payments.len());

        for payment in payments.into_iter() {
            new_arg_buffer.push_arg(payment.token_identifier);
            new_arg_buffer.push_arg(payment.token_nonce);
            new_arg_buffer.push_arg(payment.amount);
        }
        if !self.basic.endpoint_name.is_empty() {
            new_arg_buffer.push_arg(self.basic.endpoint_name);
        }

        ContractCallWithKlv {
            basic: ContractCallNoPayment {
                _phantom: PhantomData,
                to: self.basic.to,
                endpoint_name: KLEVER_TRANSFER_FUNC_NAME.into(),
                arg_buffer: new_arg_buffer.concat(self.basic.arg_buffer),
                explicit_gas_limit: self.basic.explicit_gas_limit,
                _return_type: PhantomData,
            },
            klv_payment: BigUint::zero(),
        }
    }
}
