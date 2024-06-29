use crate::{
    api::{ManagedTypeApi, RawHandle},
    codec,
    codec::{
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
        *,
    },
    contract_base::ExitCodecErrorHandler,
    types::{
        BigUint, ManagedBuffer, ManagedType,
    },
    derive::TypeAbi,
};

use crate as klever_sc; // needed by the TypeAbi generated code

const DECODE_ATTRIBUTE_ERROR_PREFIX: &[u8] = b"error decoding SFT attributes: ";

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
            max_supply: BigUint::from_raw_handle(max_supply_handle),
            circulation_supply: BigUint::from_raw_handle(circulation_supply_handle),
            metadata: SFTMetadata::from(ManagedBuffer::from_raw_handle(meta_handle)),
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


impl<M: ManagedTypeApi> SFTMetadata<M> {
    pub fn try_decode_attributes<T: TopDecode>(&self) -> Result<T, DecodeError> {
        T::top_decode(self.attributes.clone())
    }

    pub fn decode_attributes<T: TopDecode>(&self) -> T {
        let Ok(value) = T::top_decode_or_handle_err(
            self.attributes.clone(),
            ExitCodecErrorHandler::<M>::from(DECODE_ATTRIBUTE_ERROR_PREFIX),
        );
        value
    }
}


pub fn get_raw_handle<M: ManagedTypeApi>(mb: &ManagedBuffer<M>, pos: usize) -> RawHandle {
    let mut dest_slice = [0u8; 4];
    mb.load_slice(pos, &mut dest_slice).unwrap();

    RawHandle::from_be_bytes(dest_slice)
}