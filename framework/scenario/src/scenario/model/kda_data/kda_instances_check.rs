use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckKdaInstancesRaw,
};

use super::CheckKdaInstance;

#[derive(Debug, Clone)]
pub enum CheckKdaInstances {
    Star,
    Equal(Vec<CheckKdaInstance>),
}

impl CheckKdaInstances {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckKdaInstances::Star)
    }

    pub fn contains_nonce(&self, nonce: u64) -> bool {
        match &self {
            CheckKdaInstances::Equal(eq) => {
                for expected_value in eq.iter() {
                    if expected_value.nonce.value == nonce {
                        return true;
                    }
                }
            },
            CheckKdaInstances::Star => {},
        }
        false
    }
}

impl Default for CheckKdaInstances {
    fn default() -> Self {
        CheckKdaInstances::Equal(Vec::new())
    }
}

impl InterpretableFrom<CheckKdaInstancesRaw> for CheckKdaInstances {
    fn interpret_from(from: CheckKdaInstancesRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckKdaInstancesRaw::Unspecified => CheckKdaInstances::Star,
            CheckKdaInstancesRaw::Star => CheckKdaInstances::Star,
            CheckKdaInstancesRaw::Equal(m) => CheckKdaInstances::Equal(
                m.into_iter()
                    .map(|v| CheckKdaInstance::interpret_from(v, context))
                    .collect(),
            ),
        }
    }
}

impl IntoRaw<CheckKdaInstancesRaw> for CheckKdaInstances {
    fn into_raw(self) -> CheckKdaInstancesRaw {
        match self {
            CheckKdaInstances::Equal(eq) => {
                CheckKdaInstancesRaw::Equal(eq.into_iter().map(|cei| cei.into_raw()).collect())
            },
            CheckKdaInstances::Star => CheckKdaInstancesRaw::Star,
        }
    }
}
