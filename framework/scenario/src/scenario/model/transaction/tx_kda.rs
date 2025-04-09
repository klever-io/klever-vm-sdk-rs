use klever_sc::api::ManagedTypeApi;
use klever_sc::types::KdaTokenPayment;

use crate::{
    scenario::model::{BigUintValue, BytesValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::{TxKDARaw, ValueSubTree},
    },
};

#[derive(Debug, Clone)]
pub struct TxKDA {
    pub kda_token_identifier: BytesValue,
    pub nonce: U64Value,
    pub kda_value: BigUintValue,
}

impl InterpretableFrom<TxKDARaw> for TxKDA {
    fn interpret_from(from: TxKDARaw, context: &InterpreterContext) -> Self {
        TxKDA {
            kda_token_identifier: interpret_kda_token_identifier(from.token_identifier, context),
            nonce: interpret_opt_u64(from.nonce.clone(), context),
            kda_value: interpret_opt_big_uint(from.value, from.nonce, context),
        }
    }
}

impl IntoRaw<TxKDARaw> for TxKDA {
    fn into_raw(self) -> TxKDARaw {
        TxKDARaw {
            token_identifier: Some(self.kda_token_identifier.into_raw()),
            nonce: self.nonce.into_raw_opt(),
            value: self.kda_value.into_raw_opt(),
        }
    }
}

impl<M: ManagedTypeApi> From<KdaTokenPayment<M>> for TxKDA {
    fn from(value: KdaTokenPayment<M>) -> Self {
        TxKDA {
            kda_token_identifier: BytesValue::from(
                value.token_identifier.as_managed_buffer().to_vec(),
            ),
            nonce: U64Value::from(value.token_nonce),
            kda_value: BigUintValue::from(value.amount),
        }
    }
}

fn interpret_kda_token_identifier(
    kda_token_identifier: Option<ValueSubTree>,
    context: &InterpreterContext,
) -> BytesValue {
    if let Some(kda_token_identifier_raw) = kda_token_identifier {
        BytesValue::interpret_from(kda_token_identifier_raw, context)
    } else {
        BytesValue::empty()
    }
}

fn interpret_opt_u64(opt_u64: Option<ValueSubTree>, context: &InterpreterContext) -> U64Value {
    if let Some(u) = opt_u64 {
        U64Value::interpret_from(u, context)
    } else {
        U64Value::empty()
    }
}

fn interpret_opt_big_uint(
    opt_big_uint: Option<ValueSubTree>,
    opt_u64: Option<ValueSubTree>,
    context: &InterpreterContext,
) -> BigUintValue {
    if let Some(u) = opt_big_uint {
        BigUintValue::interpret_from(u, context)
    } else {
        match opt_u64 {
            Some(_) => BigUintValue::from(1u64),
            None => BigUintValue::default(),
        }
    }
}
