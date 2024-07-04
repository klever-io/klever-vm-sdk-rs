use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{BigUint, KdaTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec, TokenIdentifier},
};
use crate::types::ContractCallBase;

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithKlv};

#[must_use]
pub struct ContractCallWithMultiKda<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub kda_payments: ManagedVec<SA, KdaTokenPayment<SA>>,
}

impl<SA, OriginalResult> ContractCallBase<SA> for ContractCallWithMultiKda<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithKlv<SA, Self::OriginalResult> {
        self.basic
            .into_normalized()
            .convert_to_kda_transfer_call(self.kda_payments)
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithMultiKda<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        self.basic.transfer_execute_kda(self.kda_payments);
    }
}

impl<SA, OriginalResult> ContractCallWithMultiKda<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_kda_transfer` or `with_multi_token_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        payments: ManagedVec<SA, KdaTokenPayment<SA>>,
    ) -> Self {
        ContractCallWithMultiKda {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            kda_payments: payments,
        }
    }

    /// Adds a single KDA token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_kda_transfer<P: Into<KdaTokenPayment<SA>>>(mut self, payment: P) -> Self {
        self.kda_payments.push(payment.into());
        self
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_kda_transfer((payment_token, payment_nonce, payment_amount))`. 
        The tuple argument will get automatically converted to KdaTokenPayment."
    )]
    pub fn add_kda_token_transfer(
        self,
        payment_token: TokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> Self {
        self.with_kda_transfer((
            payment_token,
            payment_nonce,
            payment_amount,
        ))
    }
}
