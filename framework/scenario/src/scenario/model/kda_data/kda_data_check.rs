use crate::{
    scenario::model::{CheckValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::CheckKdaDataRaw,
    },
};

use super::CheckKdaInstances;

#[derive(Debug, Default, Clone)]
pub struct CheckKdaData {
    pub instances: CheckKdaInstances,
    pub last_nonce: CheckValue<U64Value>,
    pub frozen: CheckValue<U64Value>,
}

impl InterpretableFrom<CheckKdaDataRaw> for CheckKdaData {
    fn interpret_from(from: CheckKdaDataRaw, context: &InterpreterContext) -> Self {
        CheckKdaData {
            instances: CheckKdaInstances::interpret_from(from.instances, context),
            last_nonce: CheckValue::<U64Value>::interpret_from(from.last_nonce, context),
            frozen: CheckValue::<U64Value>::interpret_from(from.frozen, context),
        }
    }
}

impl IntoRaw<CheckKdaDataRaw> for CheckKdaData {
    fn into_raw(self) -> CheckKdaDataRaw {
        CheckKdaDataRaw {
            instances: self.instances.into_raw(),
            last_nonce: self.last_nonce.into_raw(),
            roles: Vec::new(),
            frozen: self.frozen.into_raw(),
        }
    }
}
