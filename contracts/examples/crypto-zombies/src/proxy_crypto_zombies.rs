// Code generated by the klever-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use klever_sc::proxy_imports::*;

pub struct CryptoZombiesProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for CryptoZombiesProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = CryptoZombiesProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        CryptoZombiesProxyMethods { wrapped_tx: tx }
    }
}

pub struct CryptoZombiesProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> CryptoZombiesProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> CryptoZombiesProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> CryptoZombiesProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn set_crypto_kitties_sc_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("set_crypto_kitties_sc_address")
            .argument(&address)
            .original_result()
    }

    pub fn generate_random_dna(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("generate_random_dna")
            .original_result()
    }

    pub fn create_random_zombie<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        name: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("create_random_zombie")
            .argument(&name)
            .original_result()
    }

    pub fn is_ready<
        Arg0: ProxyArg<usize>,
    >(
        self,
        zombie_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("is_ready")
            .argument(&zombie_id)
            .original_result()
    }

    pub fn feed_on_kitty<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<u32>,
    >(
        self,
        zombie_id: Arg0,
        kitty_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("feed_on_kitty")
            .argument(&zombie_id)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn dna_digits(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u8> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("dna_digits")
            .original_result()
    }

    pub fn zombie_last_index(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("zombie_last_index")
            .original_result()
    }

    pub fn zombies<
        Arg0: ProxyArg<usize>,
    >(
        self,
        id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Zombie<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("zombies")
            .argument(&id)
            .original_result()
    }

    pub fn zombie_owner<
        Arg0: ProxyArg<usize>,
    >(
        self,
        id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("zombie_owner")
            .argument(&id)
            .original_result()
    }

    pub fn crypto_kitties_sc_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("crypto_kitties_sc_address")
            .original_result()
    }

    pub fn cooldown_time(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("cooldown_time")
            .original_result()
    }

    pub fn owned_zombies<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        owner: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, usize>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("owned_zombies")
            .argument(&owner)
            .original_result()
    }

    pub fn level_up<
        Arg0: ProxyArg<usize>,
    >(
        self,
        zombie_id: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("level_up")
            .argument(&zombie_id)
            .original_result()
    }

    pub fn withdraw(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("withdraw")
            .original_result()
    }

    pub fn change_name<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        zombie_id: Arg0,
        name: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("change_name")
            .argument(&zombie_id)
            .argument(&name)
            .original_result()
    }

    pub fn change_dna<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        zombie_id: Arg0,
        dna: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("change_dna")
            .argument(&zombie_id)
            .argument(&dna)
            .original_result()
    }

    pub fn attack<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<usize>,
    >(
        self,
        zombie_id: Arg0,
        target_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("attack")
            .argument(&zombie_id)
            .argument(&target_id)
            .original_result()
    }
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Zombie<Api>
where
    Api: ManagedTypeApi,
{
    pub name: ManagedBuffer<Api>,
    pub dna: u64,
    pub level: u16,
    pub ready_time: u64,
    pub win_count: usize,
    pub loss_count: usize,
}
