use klever_sc::imports::*;
use klever_sc::derive_imports::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub kda_funds: ManagedVec<M, KdaTokenPayment<M>>,
    pub klv_funds: BigUint<M>,
    pub valability: u64,
    pub expiration_round: u64,
    pub fees: Fee<M>,
}

impl<M> DepositInfo<M>
where
    M: ManagedTypeApi,
{
    pub fn get_num_tokens(&self) -> usize {
        let mut amount = self.kda_funds.len();
        if self.klv_funds > 0 {
            amount += 1;
        }

        amount
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Fee<M: ManagedTypeApi> {
    pub num_token_to_transfer: usize,
    pub value: KdaTokenPayment<M>,
}
