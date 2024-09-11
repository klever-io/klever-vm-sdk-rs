use core::marker::PhantomData;

use crate::{
    api::CallTypeApi,
    types::{BigUint, KdaTokenPayment, ManagedVec},
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCallWithKlv};

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
        ContractCallWithKlv {
            basic: ContractCallNoPayment {
                _phantom: PhantomData,
                to: self.basic.to.clone(),
                function_call: self
                    .basic
                    .function_call
                    .convert_to_multi_transfer_kda_call(&self.basic.to, &payments),
                explicit_gas_limit: self.basic.explicit_gas_limit,
                _return_type: PhantomData,
            },
            klv_payment: BigUint::zero(),
        }
    }
}
