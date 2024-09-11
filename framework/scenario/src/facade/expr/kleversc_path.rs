use klever_chain_scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};
use klever_sc::types::{AnnotatedValue, ManagedBuffer, TxCodeValue};

use crate::{ScenarioTxEnv, ScenarioTxEnvData};

use super::RegisterCodeSource;

const KLEVERSC_PREFIX: &str = "kleversc:";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KleverscPath<'a> {
    path: &'a str,
}

impl<'a> KleverscPath<'a> {
    pub const fn new(path: &'a str) -> Self {
        KleverscPath { path }
    }
}

impl<'a> KleverscPath<'a> {
    pub fn eval_to_expr(&self) -> String {
        format!("{KLEVERSC_PREFIX}{}", self.path)
    }

    pub fn resolve_contents(&self, context: &InterpreterContext) -> Vec<u8> {
        interpret_string(&format!("{KLEVERSC_PREFIX}{}", self.path), context)
    }
}

impl<'a, Env> AnnotatedValue<Env, ManagedBuffer<Env::Api>> for KleverscPath<'a>
where
    Env: ScenarioTxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.eval_to_expr().into()
    }

    fn to_value(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        self.resolve_contents(&env.env_data().interpreter_context())
            .into()
    }
}

impl<'a, Env> TxCodeValue<Env> for KleverscPath<'a> where Env: ScenarioTxEnv {}

impl<'a> RegisterCodeSource for KleverscPath<'a> {
    fn into_code(self, env_data: ScenarioTxEnvData) -> Vec<u8> {
        self.resolve_contents(&env_data.interpreter_context())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::imports::KleverscPath;

    fn assert_eq_eval(expr: &'static str, expected: &str) {
        assert_eq!(&KleverscPath::new(expr).eval_to_expr(), expected);
    }

    #[test]
    fn test_address_value() {
        assert_eq_eval(
            "output/adder.kleversc.json",
            "kleversc:output/adder.kleversc.json",
        );
    }
}
