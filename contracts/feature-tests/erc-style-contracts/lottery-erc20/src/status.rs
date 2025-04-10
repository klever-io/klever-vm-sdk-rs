use klever_sc::derive_imports::*;

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Inactive,
    Running,
    Ended,
    DistributingPrizes,
}
