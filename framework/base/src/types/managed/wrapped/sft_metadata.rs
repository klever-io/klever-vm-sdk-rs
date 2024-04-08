use crate::{
    api::{HandleConstraints, ManagedTypeApi},
    codec::{self, derive::{NestedDecode, NestedEncode, TopDecode, TopEncode}},
    types::{get_raw_handle, BigUint, ManagedBuffer, ManagedType},
};

use crate as klever_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Debug)]
pub struct SFTMeta<M: ManagedTypeApi> {
    pub max_supply: BigUint<M>,
    pub circulation_supply: BigUint<M>,
    pub metadata: SFTMetadata<M>,
}

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Debug)]
pub struct SFTMetadata<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub hash: ManagedBuffer<M>,
    pub attributes: ManagedBuffer<M>,
}

impl<M> From<ManagedBuffer<M>> for SFTMeta<M>
where
    M: ManagedTypeApi,
{
    fn from(v: ManagedBuffer<M>) -> Self {
        let max_supply_handle = get_raw_handle(&v, 0);
        let circulation_supply_handle = get_raw_handle(&v, 4);
        let meta_handle = get_raw_handle(&v, 8);

        SFTMeta{
            max_supply: BigUint::from_raw_handle(max_supply_handle.get_raw_handle()),
            circulation_supply: BigUint::from_raw_handle(circulation_supply_handle.get_raw_handle()),
            metadata: SFTMetadata::from(ManagedBuffer::from_raw_handle(meta_handle.get_raw_handle())),
        }
    }
}

impl<M> From<ManagedBuffer<M>> for SFTMetadata<M>
where
    M: ManagedTypeApi,
{
    fn from(v: ManagedBuffer<M>) -> Self {
        let name_handle = get_raw_handle(&v, 0);
        let hash_handle = get_raw_handle(&v, 4);
        let attributes_handle = get_raw_handle(&v, 8);

        SFTMetadata {
            name: ManagedBuffer::from_raw_handle(name_handle),
            hash: ManagedBuffer::from_raw_handle(hash_handle),
            attributes: ManagedBuffer::from_raw_handle(attributes_handle),
        }
    }
}