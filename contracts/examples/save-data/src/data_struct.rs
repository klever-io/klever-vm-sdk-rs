use klever_sc::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedVec, TokenIdentifier},
};

klever_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DataStruct<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub managed_vec: ManagedVec<M, u8>,
    pub big_number: BigUint<M>,
    pub number: u64,
}
