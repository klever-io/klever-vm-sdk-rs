use super::{KdaTokenPayment, ManagedVec};
use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    types::BigUint,
};

use crate as klever_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

/// Encodes any type of payment, which either:
/// - KLV (can be zero in case of no payment whatsoever);
/// - Multi-KDA (one or more KDA transfers).
#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub enum KlvOrMultiKdaPayment<M: ManagedTypeApi> {
    Klv(BigUint<M>),
    MultiKda(ManagedVec<M, KdaTokenPayment<M>>),
}

impl<M: ManagedTypeApi> KlvOrMultiKdaPayment<M> {
    pub fn is_empty(&self) -> bool {
        match self {
            KlvOrMultiKdaPayment::Klv(klv_value) => klv_value == &0u32,
            KlvOrMultiKdaPayment::MultiKda(kda_payments) => kda_payments.is_empty(),
        }
    }
}

/// The version of `KlvOrMultiKdaPayment` that contains referrences instead of owned fields.
pub enum KlvOrMultiKdaPaymentRefs<'a, M: ManagedTypeApi> {
    Klv(&'a BigUint<M>),
    MultiKda(&'a ManagedVec<M, KdaTokenPayment<M>>),
}

impl<M: ManagedTypeApi> KlvOrMultiKdaPayment<M> {
    pub fn as_refs(&self) -> KlvOrMultiKdaPaymentRefs<'_, M> {
        match self {
            KlvOrMultiKdaPayment::Klv(klv_value) => {
                KlvOrMultiKdaPaymentRefs::Klv(klv_value)
            },
            KlvOrMultiKdaPayment::MultiKda(kda_payments) => {
                KlvOrMultiKdaPaymentRefs::MultiKda(kda_payments)
            },
        }
    }
}

impl<'a, M: ManagedTypeApi> KlvOrMultiKdaPaymentRefs<'a, M> {
    pub fn to_owned_payment(&self) -> KlvOrMultiKdaPayment<M> {
        match self {
            KlvOrMultiKdaPaymentRefs::Klv(klv_value) => {
                KlvOrMultiKdaPayment::Klv((*klv_value).clone())
            },
            KlvOrMultiKdaPaymentRefs::MultiKda(kda_payments) => {
                KlvOrMultiKdaPayment::MultiKda((*kda_payments).clone())
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            KlvOrMultiKdaPaymentRefs::Klv(klv_value) => *klv_value == &0u32,
            KlvOrMultiKdaPaymentRefs::MultiKda(kda_payments) => kda_payments.is_empty(),
        }
    }
}
