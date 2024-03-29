use super::{KdaTokenPayment, ManagedVec};
use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
        CodecFromSelf,
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

impl<M> CodecFromSelf for KlvOrMultiKdaPayment<M> where M: ManagedTypeApi {}
