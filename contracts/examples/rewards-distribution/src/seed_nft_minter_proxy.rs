// Code generated by the klever-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use klever_sc::proxy_imports::*;

pub struct SeedNftMinterProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for SeedNftMinterProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = SeedNftMinterProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        SeedNftMinterProxyMethods { wrapped_tx: tx }
    }
}

pub struct SeedNftMinterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> SeedNftMinterProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<ManagedVec<Env::Api, ManagedAddress<Env::Api>>>,
        Arg1: ProxyArg<ManagedVec<Env::Api, Distribution<Env::Api>>>,
    >(
        self,
        marketplaces: Arg0,
        distribution: Arg1,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&marketplaces)
            .argument(&distribution)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> SeedNftMinterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn create_nft<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
        Arg4: ProxyArg<OptionalValue<TokenIdentifier<Env::Api>>>,
        Arg5: ProxyArg<OptionalValue<u64>>,
    >(
        self,
        name: Arg0,
        royalties: Arg1,
        uri: Arg2,
        selling_price: Arg3,
        opt_token_used_as_payment: Arg4,
        opt_token_used_as_payment_nonce: Arg5,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("createNft")
            .argument(&name)
            .argument(&royalties)
            .argument(&uri)
            .argument(&selling_price)
            .argument(&opt_token_used_as_payment)
            .argument(&opt_token_used_as_payment_nonce)
            .original_result()
    }

    pub fn claim_and_distribute<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        token_id: Arg0,
        token_nonce: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claimAndDistribute")
            .argument(&token_id)
            .argument(&token_nonce)
            .original_result()
    }

    pub fn marketplaces(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getMarketplaces")
            .original_result()
    }

    pub fn nft_count(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftCount")
            .original_result()
    }

    pub fn distribution_rules(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Distribution<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDistributionRules")
            .original_result()
    }

    pub fn issue_token<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("issueToken")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .original_result()
    }

    pub fn buy_nft<
        Arg0: ProxyArg<u64>,
    >(
        self,
        nft_nonce: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("buyNft")
            .argument(&nft_nonce)
            .original_result()
    }

    pub fn get_nft_price<
        Arg0: ProxyArg<u64>,
    >(
        self,
        nft_nonce: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<MultiValue3<TokenIdentifier<Env::Api>, u64, BigUint<Env::Api>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftPrice")
            .argument(&nft_nonce)
            .original_result()
    }

    pub fn nft_token_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftTokenId")
            .original_result()
    }
}

#[type_abi]
#[derive(ManagedVecItem, NestedEncode, NestedDecode)]
pub struct Distribution<Api>
where
    Api: ManagedTypeApi,
{
    pub address: ManagedAddress<Api>,
    pub percentage: u64,
    pub endpoint: ManagedBuffer<Api>,
    pub gas_limit: u64,
}
