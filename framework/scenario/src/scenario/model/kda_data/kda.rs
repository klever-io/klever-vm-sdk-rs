use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::{KdaFullRaw, KdaRaw},
};

use super::{KdaInstance, KdaObject};
use crate::scenario::model::{BigUintValue, BytesValue, U64Value};

#[derive(Debug, Clone)]
pub enum Kda {
    Short(BigUintValue),
    Full(KdaObject),
}

impl Kda {
    pub fn convert_to_short_if_possible(&mut self) {
        if let Kda::Full(kda_obj) = self {
            if kda_obj.is_short_form() {
                *self = Self::Short(kda_obj.instances[0].balance.clone().unwrap())
            }
        }
    }

    pub fn convert_to_full(&mut self) {
        if let Kda::Short(balance) = self {
            let mut new_kda_obj = KdaObject::default();
            new_kda_obj.set_balance(0u64, balance.clone());

            *self = Self::Full(new_kda_obj);
        }
    }

    pub fn set_balance<N, A>(&mut self, token_nonce_expr: N, amount_expr: A)
    where
        U64Value: From<N>,
        BigUintValue: From<A>,
    {
        self.convert_to_full();

        if let Kda::Full(kda_obj) = self {
            kda_obj.set_balance(token_nonce_expr, amount_expr);
        }
    }

    pub fn get_mut_kda_object(&mut self) -> &mut KdaObject {
        self.convert_to_full();

        if let Kda::Full(kda_obj) = self {
            return kda_obj;
        }

        unreachable!()
    }
}

impl InterpretableFrom<KdaRaw> for Kda {
    fn interpret_from(from: KdaRaw, context: &InterpreterContext) -> Self {
        match from {
            KdaRaw::Short(short_kda) => {
                Kda::Short(BigUintValue::interpret_from(short_kda, context))
            },
            KdaRaw::Full(full_kda) => Kda::Full(KdaObject {
                token_identifier: full_kda
                    .token_identifier
                    .map(|b| BytesValue::interpret_from(b, context)),
                instances: full_kda
                    .instances
                    .into_iter()
                    .map(|instance| KdaInstance::interpret_from(instance, context))
                    .collect(),
                last_nonce: full_kda
                    .last_nonce
                    .map(|b| U64Value::interpret_from(b, context)),
                roles: full_kda.roles,
                frozen: full_kda
                    .frozen
                    .map(|b| U64Value::interpret_from(b, context)),
            }),
        }
    }
}

impl IntoRaw<KdaRaw> for Kda {
    fn into_raw(mut self) -> KdaRaw {
        self.convert_to_short_if_possible();

        match self {
            Kda::Short(short) => KdaRaw::Short(short.original),
            Kda::Full(eo) => KdaRaw::Full(KdaFullRaw {
                token_identifier: eo.token_identifier.map(|ti| ti.original),
                instances: eo
                    .instances
                    .into_iter()
                    .map(|inst| inst.into_raw())
                    .collect(),
                last_nonce: eo.last_nonce.map(|ti| ti.original),
                roles: eo.roles,
                frozen: eo.frozen.map(|ti| ti.original),
            }),
        }
    }
}
