use crate::{
    api::ManagedTypeApi,
    codec,
    codec::{
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
        *,
    },
    contract_base::ExitCodecErrorHandler,
    derive::{ManagedVecItem, TypeAbi},
    types::{BigUint, get_u32, get_raw_handle, ManagedBuffer, ManagedType, ManagedVec},
};

use crate as klever_sc; // needed by the TypeAbi generated code

const DECODE_ATTRIBUTE_ERROR_PREFIX: &[u8] = b"error decoding NFT attributes: ";

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Debug)]
pub struct UserKDA<M: ManagedTypeApi> {
    pub balance: BigUint<M>,
    pub frozen_balance: BigUint<M>,
    pub last_claim: LastClaim<M>,
    pub buckets: ManagedVec<M, UserBucket<M>>,
    pub mime: ManagedBuffer<M>,
    pub metadata: ManagedBuffer<M>,
}

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Debug)]
pub struct LastClaim<M: ManagedTypeApi> {
    pub timestamp: BigUint<M>,
    pub epoch: u32,
}

impl<M> From<ManagedBuffer<M>> for LastClaim<M>
where
    M: ManagedTypeApi,
{
    fn from(v: ManagedBuffer<M>) -> Self {
        let timestamp_handle = get_raw_handle(&v, 0);

        LastClaim {
            timestamp: BigUint::from_raw_handle(timestamp_handle),
            epoch: get_u32(&v, 4),
        }
    }
}

#[derive(
    ManagedVecItem, Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Debug,
)]
pub struct UserBucket<M: ManagedTypeApi> {
    pub key: ManagedBuffer<M>,
    pub staked_at: BigUint<M>,
    pub staked_epoch: u32,
    pub unstaked_epoch: u32,
    pub value: BigUint<M>,
    pub delegation: ManagedBuffer<M>,
}


impl<M: ManagedTypeApi> UserKDA<M> {
    pub fn try_decode_attributes<T: TopDecode>(&self) -> Result<T, DecodeError> {
        T::top_decode(self.metadata.clone())
    }

    pub fn decode_attributes<T: TopDecode>(&self) -> T {
        let Ok(value) = T::top_decode_or_handle_err(
            self.metadata.clone(),
            ExitCodecErrorHandler::<M>::from(DECODE_ATTRIBUTE_ERROR_PREFIX),
        );
        value
    }
}