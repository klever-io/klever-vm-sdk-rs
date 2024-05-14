use core::marker::PhantomData;

use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, KdaTokenPayment,
        KlvOrMultiKdaPayment, ManagedAddress, ManagedBuffer, ManagedVec, TokenIdentifier,
    },
};
use crate::types::FunctionCall;

use super::{
    contract_call_exec::UNSPECIFIED_GAS_LIMIT, contract_call_with_klv::ContractCallWithKlv,
    contract_call_with_multi_kda::ContractCallWithMultiKda, ContractCall,
    ContractCallWithAnyPayment, ContractCallWithKlvOrSingleKda, ManagedArgBuffer,
};

/// Holds metadata for calling another contract, without payments.
///
/// Proxies generally create contract calls of this type
/// (unless there are payment arguments in the endpoint - but these are mostly obsolete now).
///
/// It is also the basis for all other contract call types, all of them contain this one.
#[must_use]
pub struct ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) _phantom: PhantomData<SA>,
    pub to: ManagedAddress<SA>,
    pub function_call: FunctionCall<SA>,
    pub explicit_gas_limit: u64,
    pub(super) _return_type: PhantomData<OriginalResult>,
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    #[inline]
    fn into_normalized(self) -> ContractCallWithKlv<SA, Self::OriginalResult> {
        ContractCallWithKlv {
            basic: self,
            klv_payment: BigUint::zero(),
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        self
    }

    fn transfer_execute(self) {
        let kda_payments: ManagedVec<SA, KdaTokenPayment<SA>> = ManagedVec::new();
        self.transfer_execute_kda(kda_payments);
    }
}

impl<SA, OriginalResult> ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn new<N: Into<ManagedBuffer<SA>>>(to: ManagedAddress<SA>, endpoint_name: N) -> Self {
        ContractCallNoPayment {
            _phantom: PhantomData,
            to,
            function_call: FunctionCall {
                function_name: endpoint_name.into(),
                arg_buffer: ManagedArgBuffer::new(),
            },
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            _return_type: PhantomData,
        }
    }

    /// Sets payment to be KLV transfer.
    pub fn with_klv_transfer(
        self,
        klv_amount: BigUint<SA>,
    ) -> ContractCallWithKlv<SA, OriginalResult> {
        ContractCallWithKlv {
            basic: self,
            klv_payment: klv_amount,
        }
    }

    /// Adds a single KDA token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_kda_transfer<P: Into<KdaTokenPayment<SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithMultiKda<SA, OriginalResult> {
        let result = ContractCallWithMultiKda {
            basic: self,
            kda_payments: ManagedVec::new(),
        };
        result.with_kda_transfer(payment)
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_multi_token_transfer(
        self,
        payments: ManagedVec<SA, KdaTokenPayment<SA>>,
    ) -> ContractCallWithMultiKda<SA, OriginalResult> {
        ContractCallWithMultiKda {
            basic: self,
            kda_payments: payments,
        }
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_any_payment(
        self,
        payment: KlvOrMultiKdaPayment<SA>,
    ) -> ContractCallWithAnyPayment<SA, OriginalResult> {
        ContractCallWithAnyPayment {
            basic: self,
            payment,
        }
    }

    /// Sets payment to be either KLV or a single KDA transfer, as determined at runtime.
    pub fn with_klv_or_single_kda_transfer<P: Into<KdaTokenPayment<SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithKlvOrSingleKda<SA, OriginalResult> {
        ContractCallWithKlvOrSingleKda {
            basic: self,
            payment: payment.into(),
        }
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_klv_or_single_kda_transfer((payment_token, payment_nonce, payment_amount))`. "
    )]
    pub fn with_klv_or_single_kda_token_transfer(
        self,
        payment_token: TokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> ContractCallWithKlvOrSingleKda<SA, OriginalResult> {
        self.with_klv_or_single_kda_transfer((
            payment_token,
            payment_nonce,
            payment_amount,
        ))
    }

    pub fn into_function_call(self) -> FunctionCall<SA> {
        self.function_call
    }
}
