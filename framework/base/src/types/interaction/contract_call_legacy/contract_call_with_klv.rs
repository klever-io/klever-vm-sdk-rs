use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{BigUint, KdaTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec, TokenIdentifier},
};
use crate::types::ContractCallBase;

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall};

/// Holds data for calling another contract, with KLV payment only.
///
/// Gets created when chaining method `with_klv_transfer`.
///
/// If the payment is zero, it bevahes exactly like `ContractCallNoPayment`.
///
/// It also represents the normalized form of any contract call, since KDA transfers
/// (the only payment not available here) get converted to builtin function calls in normalized form.
#[must_use]
pub struct ContractCallWithKlv<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub klv_payment: BigUint<SA>,
}

impl<SA, OriginalResult> ContractCallBase<SA> for ContractCallWithKlv<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    #[inline]
    fn into_normalized(self) -> ContractCallWithKlv<SA, Self::OriginalResult> {
        // no KDA, no conversion needed
        self
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithKlv<SA, OriginalResult>
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

        kda_payments.push(KdaTokenPayment {
            token_identifier: TokenIdentifier::from("KLV"),
            token_nonce: 0,
            amount: self.klv_payment,
        });

        self.basic.transfer_execute_kda(kda_payments);
    }
}

impl<SA, OriginalResult> ContractCallWithKlv<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_klv_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        klv_payment: BigUint<SA>,
    ) -> Self {
        ContractCallWithKlv {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            klv_payment,
        }
    }
}
