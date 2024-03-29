use crate::{
    scenario::model::{AddressValue, BigUintValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::TxTransferRaw,
    },
};

use super::{tx_interpret_util::interpret_klv_value, TxCall, TxKDA};

#[derive(Debug, Default, Clone)]
pub struct TxTransfer {
    pub from: AddressValue,
    pub to: AddressValue,
    pub klv_value: BigUintValue,
    pub kda_value: Vec<TxKDA>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxTransferRaw> for TxTransfer {
    fn interpret_from(from: TxTransferRaw, context: &InterpreterContext) -> Self {
        TxTransfer {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            klv_value: interpret_klv_value(from.value, from.klv_value, context),
            kda_value: from
                .kda_value
                .iter()
                .map(|kda_value| TxKDA::interpret_from(kda_value.clone(), context))
                .collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit.unwrap_or_default(), context),
            gas_price: U64Value::interpret_from(from.gas_price.unwrap_or_default(), context),
        }
    }
}

impl IntoRaw<TxTransferRaw> for TxTransfer {
    fn into_raw(self) -> TxTransferRaw {
        TxTransferRaw {
            from: self.from.into_raw(),
            to: self.to.into_raw(),
            value: None,
            klv_value: self.klv_value.into_raw_opt(),
            kda_value: self
                .kda_value
                .into_iter()
                .map(|kda_value| kda_value.into_raw())
                .collect(),
            gas_limit: self.gas_limit.into_raw_opt(),
            gas_price: self.gas_price.into_raw_opt(),
        }
    }
}

impl TxTransfer {
    /// Converts to a TxCall, with empty endpoint and arguments.
    pub fn to_tx_call(&self) -> TxCall {
        TxCall {
            from: self.from.clone(),
            to: self.to.clone(),
            klv_value: self.klv_value.clone(),
            kda_value: self.kda_value.clone(),
            function: String::new(),
            arguments: Vec::new(),
            gas_limit: self.gas_limit.clone(),
            gas_price: self.gas_price.clone(),
        }
    }
}
