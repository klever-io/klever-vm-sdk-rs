use crate::{
    api::StaticApi,
    klever_sc::types::{ContractCall, ContractCallWithKlv, KdaTokenPayment},
    scenario::model::{AddressValue, BigUintValue, BytesValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::TxCallRaw,
    },
};

use super::{tx_interpret_util::interpret_klv_value, TxKDA};

pub const DEFAULT_GAS_EXPR: &str = "5,000,000";

#[derive(Debug, Clone)]
pub struct TxCall {
    pub from: AddressValue,
    pub to: AddressValue,
    pub klv_value: BigUintValue,
    pub kda_value: Vec<TxKDA>,
    pub function: String,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl Default for TxCall {
    fn default() -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            klv_value: Default::default(),
            kda_value: Default::default(),
            function: Default::default(),
            arguments: Default::default(),
            gas_limit: U64Value::from(DEFAULT_GAS_EXPR),
            gas_price: Default::default(),
        }
    }
}

impl InterpretableFrom<TxCallRaw> for TxCall {
    fn interpret_from(from: TxCallRaw, context: &InterpreterContext) -> Self {
        TxCall {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            klv_value: interpret_klv_value(from.value, from.klv_value, context),
            kda_value: from
                .kda_value
                .into_iter()
                .map(|kda_value| TxKDA::interpret_from(kda_value, context))
                .collect(),
            function: from.function,
            arguments: from
                .arguments
                .into_iter()
                .map(|t| BytesValue::interpret_from(t, context))
                .collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit, context),
            gas_price: U64Value::interpret_from(from.gas_price.unwrap_or_default(), context),
        }
    }
}

impl IntoRaw<TxCallRaw> for TxCall {
    fn into_raw(self) -> TxCallRaw {
        TxCallRaw {
            from: self.from.into_raw(),
            to: self.to.into_raw(),
            value: None,
            klv_value: self.klv_value.into_raw_opt(),
            kda_value: self
                .kda_value
                .into_iter()
                .map(|kda_value| kda_value.into_raw())
                .collect(),
            function: self.function,
            arguments: self
                .arguments
                .into_iter()
                .map(|arg| arg.into_raw())
                .collect(),
            gas_limit: self.gas_limit.into_raw(),
            gas_price: self.gas_price.into_raw_opt(),
        }
    }
}

impl TxCall {
    pub fn to_contract_call(&self) -> ContractCallWithKlv<StaticApi, ()> {
        let mut contract_call = ContractCallWithKlv::new(
            (&self.to.value).into(),
            self.function.as_bytes(),
            (&self.klv_value.value).into(),
        );

        contract_call.basic.explicit_gas_limit = self.gas_limit.value;

        contract_call = contract_call.convert_to_kda_transfer_call(
            self.kda_value
                .iter()
                .map(|kda| {
                    KdaTokenPayment::new(
                        kda.kda_token_identifier.value.as_slice().into(),
                        kda.nonce.value,
                        (&kda.kda_value.value).into(),
                    )
                })
                .collect(),
        );

        // For some contract calls from == to.
        // The contract call objects have no "from" field, since that is always part of the execution context.
        // On the static API there is no execution context, but a placeholder value is provided.
        // Here we already know the sender, so we can replace the placeholder with the actual value.
        if StaticApi::is_current_address_placeholder(&contract_call.basic.to.to_address()) {
            contract_call.basic.to = self.from.value.clone().into();
        }

        for argument in &self.arguments {
            contract_call.push_raw_argument(argument.value.as_slice());
        }
        contract_call
    }
}
