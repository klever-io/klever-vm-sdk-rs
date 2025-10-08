use crate::{
    api::ManagedTypeApi,
    codec,
    codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    derive::{ManagedVecItem, TypeAbi},
    types::ManagedBuffer,
};

use crate as klever_sc; // needed by the TypeAbi generated code

#[derive(
    ManagedVecItem, Clone, TopDecode, TopEncode, TypeAbi, Debug, NestedDecode, NestedEncode,
)]
pub struct ProposalParameter<M: ManagedTypeApi> {
    pub param_type: i32,
    pub value: ManagedBuffer<M>,
}
