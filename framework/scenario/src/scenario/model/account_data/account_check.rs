use klever_sc::codec::{top_encode_to_vec_u8_or_panic, TopEncode};

use crate::scenario_model::CheckKdaData;
use crate::{
    scenario::model::{
        BigUintValue, BytesKey, BytesValue, CheckKda, CheckKdaInstances, CheckKdaMap,
        CheckKdaMapContents, CheckStorage, CheckStorageDetails, CheckValue, U64Value,
    },
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::CheckAccountRaw,
    },
};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct CheckAccount {
    pub comment: Option<String>,
    pub nonce: CheckValue<U64Value>,
    pub balance: CheckValue<BigUintValue>,
    pub kda: CheckKdaMap,
    pub username: CheckValue<BytesValue>,
    pub storage: CheckStorage,
    pub code: CheckValue<BytesValue>,
    pub code_metadata: CheckValue<BytesValue>,
    pub owner: CheckValue<BytesValue>,
}

impl CheckAccount {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn nonce<V>(mut self, nonce: V) -> Self
    where
        U64Value: InterpretableFrom<V>,
    {
        self.nonce = CheckValue::Equal(U64Value::interpret_from(
            nonce,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn balance<V>(mut self, balance_expr: V) -> Self
    where
        BigUintValue: InterpretableFrom<V>,
    {
        self.balance = CheckValue::Equal(BigUintValue::interpret_from(
            balance_expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn code<V>(mut self, code_expr: V) -> Self
    where
        BytesValue: InterpretableFrom<V>,
    {
        self.code = CheckValue::Equal(BytesValue::interpret_from(
            code_expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn code_metadata<V>(mut self, code_metadata_expr: V) -> Self
    where
        BytesValue: InterpretableFrom<V>,
    {
        self.code_metadata = CheckValue::Equal(BytesValue::interpret_from(
            code_metadata_expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn kda_balance<K, V>(mut self, token_id_expr: K, balance_expr: V) -> Self
    where
        BytesKey: From<K>,
        BigUintValue: From<V>,
    {
        let token_id = BytesKey::from(token_id_expr);
        let balance = BigUintValue::from(balance_expr);

        match &mut self.kda {
            CheckKdaMap::Unspecified | CheckKdaMap::Star => {
                let mut new_kda_map = BTreeMap::new();
                let _ = new_kda_map.insert(token_id, CheckKda::Short(balance));

                let new_check_kda_map = CheckKdaMapContents {
                    contents: new_kda_map,
                    other_kdas_allowed: true,
                };

                self.kda = CheckKdaMap::Equal(new_check_kda_map);
            },
            CheckKdaMap::Equal(check_kda_map) => {
                if check_kda_map.contents.contains_key(&token_id) {
                    let prev_entry = check_kda_map.contents.get_mut(&token_id).unwrap();
                    match prev_entry {
                        CheckKda::Short(prev_balance_check) => *prev_balance_check = balance,
                        CheckKda::Full(prev_kda_check) => match prev_kda_check.instances {
                            CheckKdaInstances::Star => todo!(),
                            CheckKdaInstances::Equal(_) => todo!(),
                        },
                    }
                }
            },
        }

        self
    }

    pub fn kda_nft_balance_and_attributes<K, N, V, T>(
        mut self,
        token_id_expr: K,
        nonce_expr: N,
        balance_expr: V,
        attributes_expr: Option<T>,
    ) -> Self
    where
        BytesKey: From<K>,
        U64Value: From<N>,
        BigUintValue: From<V>,
        T: TopEncode,
    {
        let token_id = BytesKey::from(token_id_expr);

        if let CheckKdaMap::Unspecified = &self.kda {
            let mut check_kda = CheckKda::Full(CheckKdaData::default());

            if let Some(attributes_expr) = attributes_expr {
                check_kda.add_balance_and_attributes_check(
                    nonce_expr,
                    balance_expr,
                    top_encode_to_vec_u8_or_panic(&attributes_expr),
                );
            } else {
                check_kda.add_balance_and_attributes_check(
                    nonce_expr,
                    balance_expr,
                    Vec::<u8>::new(),
                );
            }

            let mut new_kda_map = BTreeMap::new();
            let _ = new_kda_map.insert(token_id, check_kda);

            let new_check_kda_map = CheckKdaMapContents {
                contents: new_kda_map,
                other_kdas_allowed: true,
            };

            self.kda = CheckKdaMap::Equal(new_check_kda_map);
        }

        self
    }

    pub fn check_storage(mut self, key: &str, value: &str) -> Self {
        let mut details = match self.storage {
            CheckStorage::Star => CheckStorageDetails::default(),
            CheckStorage::Equal(details) => details,
        };
        details.storages.insert(
            BytesKey::interpret_from(key, &InterpreterContext::default()),
            CheckValue::Equal(BytesValue::interpret_from(
                value,
                &InterpreterContext::default(),
            )),
        );
        self.storage = CheckStorage::Equal(details);
        self
    }
}

impl InterpretableFrom<Box<CheckAccountRaw>> for CheckAccount {
    fn interpret_from(from: Box<CheckAccountRaw>, context: &InterpreterContext) -> Self {
        CheckAccount {
            comment: from.comment,
            nonce: CheckValue::<U64Value>::interpret_from(from.nonce, context),
            balance: CheckValue::<BigUintValue>::interpret_from(from.balance, context),
            kda: CheckKdaMap::interpret_from(from.kda, context),
            username: CheckValue::<BytesValue>::interpret_from(from.username, context),
            storage: CheckStorage::interpret_from(from.storage, context),
            code: CheckValue::<BytesValue>::interpret_from(from.code, context),
            code_metadata: CheckValue::<BytesValue>::interpret_from(from.code_metadata, context),
            owner: CheckValue::<BytesValue>::interpret_from(from.owner, context),
        }
    }
}

impl IntoRaw<CheckAccountRaw> for CheckAccount {
    fn into_raw(self) -> CheckAccountRaw {
        CheckAccountRaw {
            comment: self.comment,
            nonce: self.nonce.into_raw(),
            balance: self.balance.into_raw(),
            kda: self.kda.into_raw(),
            username: self.username.into_raw(),
            storage: self.storage.into_raw(),
            code: self.code.into_raw_explicit(), // TODO: convert back to into_raw after VM CI upgrade
            code_metadata: self.code_metadata.into_raw(),
            owner: self.owner.into_raw_explicit(), // TODO: convert back to into_raw after VM CI upgrade
        }
    }
}
