use crate::codec::TopEncodeMulti;
use crate::types::ContractCallBase;
use crate::{
    api::CallTypeApi,
    types::{
        KdaTokenPayment, KlvOrMultiKdaPayment, ManagedAddress, ManagedBuffer, ManagedVec,
        TokenIdentifier,
    },
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithKlv};

/// Holds data for calling another contract, with any type of payment: none, KLV, Multi-KDA.
///
/// Gets created when chaining method `with_any_payment`.
#[must_use]
pub struct ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub payment: KlvOrMultiKdaPayment<SA>,
}

impl<SA, OriginalResult> ContractCallBase<SA> for ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithKlv<SA, Self::OriginalResult> {
        match self.payment {
            KlvOrMultiKdaPayment::Klv(klv_amount) => self.basic.with_klv_transfer(klv_amount),
            KlvOrMultiKdaPayment::MultiKda(multi_kda_payment) => self
                .basic
                .into_normalized()
                .convert_to_kda_transfer_call(multi_kda_payment),
        }
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        match self.payment {
            KlvOrMultiKdaPayment::Klv(klv_amount) => {
                let mut kda_payments: ManagedVec<SA, KdaTokenPayment<SA>> = ManagedVec::new();

                kda_payments.push(KdaTokenPayment {
                    token_identifier: TokenIdentifier::from("KLV"),
                    token_nonce: 0,
                    amount: klv_amount,
                });

                self.basic.transfer_execute_kda(kda_payments);
            },
            KlvOrMultiKdaPayment::MultiKda(multi_kda_payment) => {
                self.basic.transfer_execute_kda(multi_kda_payment);
            },
        }
    }
}

impl<SA, OriginalResult> ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// Creates a new instance directly.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        payment: KlvOrMultiKdaPayment<SA>,
    ) -> Self {
        ContractCallWithAnyPayment {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            payment,
        }
    }
}
