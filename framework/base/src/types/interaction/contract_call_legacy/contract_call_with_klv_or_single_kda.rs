use crate::codec::TopEncodeMulti;

use crate::types::TokenIdentifier;
use crate::{
    api::CallTypeApi,
    types::{BigUint, KdaTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec},
};

use super::{
    contract_call_no_payment::ContractCallNoPayment, contract_call_trait::ContractCallBase,
    ContractCall, ContractCallWithKlv,
};

/// Holds data for calling another contract, with a single payment, either KLV or a single KDA token.
///
/// Gets created when chaining method `with_klv_or_single_kda_transfer`.
#[must_use]
pub struct ContractCallWithKlvOrSingleKda<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub payment: KdaTokenPayment<SA>,
}

impl<SA, OriginalResult> ContractCallWithKlvOrSingleKda<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    fn into_normalized_klv(self) -> ContractCallWithKlv<SA, OriginalResult> {
        ContractCallWithKlv {
            basic: self.basic,
            klv_payment: self.payment.amount,
        }
    }

    fn into_normalized_kda(self) -> ContractCallWithKlv<SA, OriginalResult> {
        let payments = ManagedVec::from_single_item(self.payment);
        self.basic
            .into_normalized()
            .convert_to_kda_transfer_call(payments)
    }
}

impl<SA, OriginalResult> ContractCallBase<SA> for ContractCallWithKlvOrSingleKda<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithKlv<SA, Self::OriginalResult> {
        if self.payment.token_identifier.is_klv() {
            self.into_normalized_klv()
        } else {
            // Because we know that there can be at most one KDA payment,
            // there is no need to call the full `convert_to_kda_transfer_call`.
            self.into_normalized_kda()
        }
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithKlvOrSingleKda<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        let mut kda_payments: ManagedVec<SA, KdaTokenPayment<SA>> = ManagedVec::new();
        kda_payments.push(self.payment);
        self.basic.transfer_execute_kda(kda_payments)
    }
}

impl<SA, OriginalResult> ContractCallWithKlvOrSingleKda<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_klv_or_single_kda_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        token_identifier: TokenIdentifier<SA>,
        token_nonce: u64,
        amount: BigUint<SA>,
    ) -> Self {
        ContractCallWithKlvOrSingleKda {
            basic: ContractCallNoPayment::new(to.clone(), endpoint_name),
            payment: KdaTokenPayment::new(token_identifier, token_nonce, amount),
        }
    }
}
