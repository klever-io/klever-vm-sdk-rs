use klever_sc::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress, ManagedBuffer, TokenIdentifier},
};

klever_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, ManagedVecItem)]
pub struct CrowdfundingData<M: ManagedTypeApi> {
    pub id: ManagedBuffer<M>,
    pub title: ManagedBuffer<M>,
    pub logo: ManagedBuffer<M>,
    pub description: ManagedBuffer<M>,
    pub owner: ManagedAddress<M>,
    pub token: TokenIdentifier<M>,
    pub balance: BigUint<M>,
    pub claimed: BigUint<M>,
    pub target: BigUint<M>,
    pub donators: u64,
    pub deadline: u64,
}
