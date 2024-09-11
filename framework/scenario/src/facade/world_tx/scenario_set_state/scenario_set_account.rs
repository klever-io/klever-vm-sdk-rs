use std::collections::btree_map::Entry;

use klever_sc::types::{AnnotatedValue, BigUint, ManagedAddress, ManagedBuffer, TokenIdentifier};

use crate::{
    imports::StaticApi,
    scenario::tx_to_step::{
        address_annotated, big_uint_annotated, bytes_annotated, token_identifier_annotated,
        u64_annotated,
    },
    scenario_model::{Account, AddressKey, BytesKey, Kda, KdaObject, SetStateStep},
    ScenarioTxEnvData,
};

use super::{SetStateBuilder, SetStateBuilderItem};

pub struct AccountItem {
    address: AddressKey,
    account: Account,
}

impl AccountItem {
    pub fn new(address: AddressKey) -> Self {
        AccountItem {
            address,
            account: Account::default(),
        }
    }
}

impl SetStateBuilderItem for AccountItem {
    fn commit_to_step(&mut self, step: &mut SetStateStep) {
        if let Entry::Vacant(entry) = step.accounts.entry(core::mem::take(&mut self.address)) {
            entry.insert(core::mem::take(&mut self.account));
        };
    }
}

impl<'w> SetStateBuilder<'w, AccountItem> {
    pub fn nonce<N>(mut self, nonce: N) -> Self
    where
        N: AnnotatedValue<ScenarioTxEnvData, u64>,
    {
        let env = self.new_env_data();
        self.item.account.nonce = Some(u64_annotated(&env, &nonce));
        self
    }

    pub fn balance<V>(mut self, balance: V) -> Self
    where
        V: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
    {
        let env = self.new_env_data();
        self.item.account.balance = Some(big_uint_annotated(&env, &balance));
        self
    }

    pub fn kda_balance<K, V>(mut self, token_id: K, balance: V) -> Self
    where
        K: AnnotatedValue<ScenarioTxEnvData, TokenIdentifier<StaticApi>>,
        V: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
    {
        let env = self.new_env_data();
        let token_id_key = token_identifier_annotated(&env, token_id);
        let balance_value = big_uint_annotated(&env, &balance);

        let kda_data_ref = self.get_kda_data_or_create(&token_id_key);
        kda_data_ref.set_balance(0u64, balance_value);

        self
    }

    pub fn kda_nft_balance<K, N, V, T>(
        mut self,
        token_id: K,
        nonce: N,
        balance: V,
        attributes: T,
    ) -> Self
    where
        K: AnnotatedValue<ScenarioTxEnvData, TokenIdentifier<StaticApi>>,
        N: AnnotatedValue<ScenarioTxEnvData, u64>,
        V: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
        T: AnnotatedValue<ScenarioTxEnvData, ManagedBuffer<StaticApi>>,
    {
        let env = self.new_env_data();
        let token_id_key = token_identifier_annotated(&env, token_id);
        let nonce_value = u64_annotated(&env, &nonce);
        let balance_value = big_uint_annotated(&env, &balance);
        let attributes_value = bytes_annotated(&env, attributes);

        let kda_obj_ref = self
            .get_kda_data_or_create(&token_id_key)
            .get_mut_kda_object();
        kda_obj_ref.set_balance(nonce_value.clone(), balance_value);

        kda_obj_ref.set_token_attributes(nonce_value, attributes_value);

        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn kda_nft_all_properties<K, N, V, T, C, R, H, U>(
        mut self,
        token_id: K,
        nonce: N,
        balance: V,
        attributes: T,
        royalties: R,
        creator: C,
        hash: H,
        uris: Vec<U>,
    ) -> Self
    where
        K: AnnotatedValue<ScenarioTxEnvData, TokenIdentifier<StaticApi>>,
        N: AnnotatedValue<ScenarioTxEnvData, u64>,
        V: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
        T: AnnotatedValue<ScenarioTxEnvData, ManagedBuffer<StaticApi>>,
        C: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
        R: AnnotatedValue<ScenarioTxEnvData, u64>,
        H: AnnotatedValue<ScenarioTxEnvData, ManagedBuffer<StaticApi>>,
        U: AnnotatedValue<ScenarioTxEnvData, ManagedBuffer<StaticApi>>,
    {
        let env = self.new_env_data();
        let token_id_key = token_identifier_annotated(&env, token_id);
        let nonce_value = u64_annotated(&env, &nonce);
        let royalties_value = u64_annotated(&env, &royalties);
        let balance_value = big_uint_annotated(&env, &balance);
        let attributes_value = bytes_annotated(&env, attributes);
        let creator_value = address_annotated(&env, &creator);
        let hash_value = bytes_annotated(&env, hash);
        let mut uris_value = Vec::new();
        for uri in uris {
            let uri_value = bytes_annotated(&env, uri);
            uris_value.push(uri_value);
        }

        let kda_obj_ref = self
            .get_kda_data_or_create(&token_id_key)
            .get_mut_kda_object();

        kda_obj_ref.set_token_all_properties(
            nonce_value,
            balance_value,
            Some(attributes_value),
            royalties_value,
            Some(creator_value),
            Some(hash_value),
            uris_value,
        );

        self
    }

    pub fn kda_nft_last_nonce<K, N>(mut self, token_id: K, last_nonce: N) -> Self
    where
        K: AnnotatedValue<ScenarioTxEnvData, TokenIdentifier<StaticApi>>,
        N: AnnotatedValue<ScenarioTxEnvData, u64>,
    {
        let env = self.new_env_data();
        let token_id_key = token_identifier_annotated(&env, token_id);
        let nonce_value = u64_annotated(&env, &last_nonce);

        let kda_obj_ref = self
            .get_kda_data_or_create(&token_id_key)
            .get_mut_kda_object();
        kda_obj_ref.set_last_nonce(nonce_value);

        self
    }

    // TODO: Find a better way to pass roles
    pub fn kda_roles<K>(mut self, token_id: K, roles: Vec<String>) -> Self
    where
        K: AnnotatedValue<ScenarioTxEnvData, TokenIdentifier<StaticApi>>,
    {
        let env = self.new_env_data();
        let token_id_key = token_identifier_annotated(&env, token_id);

        let kda_obj_ref = self
            .get_kda_data_or_create(&token_id_key)
            .get_mut_kda_object();
        kda_obj_ref.set_roles(roles);

        self
    }

    fn get_kda_data_or_create(&mut self, token_id: &BytesKey) -> &mut Kda {
        if !self.item.account.kda.contains_key(token_id) {
            self.item
                .account
                .kda
                .insert(token_id.clone(), Kda::Full(KdaObject::default()));
        }

        self.item.account.kda.get_mut(token_id).unwrap()
    }

    pub fn code<C>(mut self, code: C) -> Self
    where
        C: AnnotatedValue<ScenarioTxEnvData, ManagedBuffer<StaticApi>>,
    {
        let env = self.new_env_data();
        let code_value = bytes_annotated(&env, code);
        self.item.account.code = Some(code_value);
        self
    }

    pub fn owner<V>(mut self, owner: V) -> Self
    where
        V: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
    {
        let env = self.new_env_data();
        let owner_value = address_annotated(&env, &owner);
        self.item.account.owner = Some(owner_value);
        self
    }
}
