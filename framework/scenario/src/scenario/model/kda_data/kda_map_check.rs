use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckKdaMapRaw,
};

use super::CheckKdaMapContents;

#[derive(Debug, Clone, Default)]
pub enum CheckKdaMap {
    #[default]
    Unspecified,
    Star,
    Equal(CheckKdaMapContents),
}

impl CheckKdaMap {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckKdaMap::Star)
    }
}

impl InterpretableFrom<CheckKdaMapRaw> for CheckKdaMap {
    fn interpret_from(from: CheckKdaMapRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckKdaMapRaw::Unspecified => CheckKdaMap::Unspecified,
            CheckKdaMapRaw::Star => CheckKdaMap::Star,
            CheckKdaMapRaw::Equal(m) => {
                CheckKdaMap::Equal(CheckKdaMapContents::interpret_from(m, context))
            },
        }
    }
}

impl IntoRaw<CheckKdaMapRaw> for CheckKdaMap {
    fn into_raw(self) -> CheckKdaMapRaw {
        match self {
            CheckKdaMap::Unspecified => CheckKdaMapRaw::Unspecified,
            CheckKdaMap::Star => CheckKdaMapRaw::Star,
            CheckKdaMap::Equal(value) => CheckKdaMapRaw::Equal(value.into_raw()),
        }
    }
}
