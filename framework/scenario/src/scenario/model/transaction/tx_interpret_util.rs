use crate::{
    scenario::model::BigUintValue,
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext},
        serde_raw::ValueSubTree,
    },
};

pub fn interpret_klv_value(
    opt_legacy_value: Option<ValueSubTree>,
    opt_klv_value: Option<ValueSubTree>,
    context: &InterpreterContext,
) -> BigUintValue {
    let mut klv_value = BigUintValue::default();
    if let Some(parsed_legacy_value) = opt_legacy_value {
        klv_value = BigUintValue::interpret_from(parsed_legacy_value, context);
    }
    if let Some(parsed_klv_value) = opt_klv_value {
        klv_value = BigUintValue::interpret_from(parsed_klv_value, context);
    }
    klv_value
}
