use crate::{
    api::ManagedTypeApi,
    codec,
    codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::{ManagedAddress, ManagedBuffer, ManagedVec},
};

use crate as klever_sc; // needed by the TypeAbi generated code
use crate::derive::{ManagedVecItem, TypeAbi};

#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    TypeAbi,
    NestedDecode,
    NestedEncode,
    TopEncode,
    TopDecode,
    ManagedVecItem,
)]
pub enum AccountPermissionType {
    Owner,
    User,
}

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Debug, ManagedVecItem)]
pub struct AccountPermission<M: ManagedTypeApi> {
    pub asset_type: AccountPermissionType,
    pub permission_name: ManagedBuffer<M>,
    pub threshold: u64,
    pub operations: ManagedBuffer<M>,
    pub signers: ManagedVec<M, AccountPermissionSigner<M>>,
}

#[derive(
    ManagedVecItem, Debug, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone,
)]
pub struct AccountPermissionSigner<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub weight: u64,
}
