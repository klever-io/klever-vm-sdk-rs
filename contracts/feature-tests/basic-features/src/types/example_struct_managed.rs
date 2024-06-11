use klever_sc::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedBuffer},
};

use klever_sc::derive_imports::*;

#[derive(
    NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Eq, Debug, Clone,
)]
pub struct ExampleStructManaged<M: ManagedTypeApi> {
    pub big_uint: BigUint<M>,
    pub int: u32,
    pub bytes: ManagedBuffer<M>,
}
