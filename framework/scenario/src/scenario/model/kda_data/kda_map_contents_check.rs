use crate::{
    scenario::model::BytesKey,
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::CheckKdaMapContentsRaw,
    },
};

use std::collections::BTreeMap;

use super::CheckKda;

#[derive(Debug, Clone)]
pub struct CheckKdaMapContents {
    pub contents: BTreeMap<BytesKey, CheckKda>,
    pub other_kdas_allowed: bool,
}

impl CheckKdaMapContents {
    pub fn contains_token(&self, token_identifier: &[u8]) -> bool {
        let token_id_conv = BytesKey::from(token_identifier.to_vec());
        self.contents.contains_key(&token_id_conv)
    }
}

impl InterpretableFrom<CheckKdaMapContentsRaw> for CheckKdaMapContents {
    fn interpret_from(from: CheckKdaMapContentsRaw, context: &InterpreterContext) -> Self {
        CheckKdaMapContents {
            contents: from
                .contents
                .into_iter()
                .map(|(k, v)| {
                    (
                        BytesKey::interpret_from(k, context),
                        CheckKda::interpret_from(v, context),
                    )
                })
                .collect(),
            other_kdas_allowed: from.other_kdas_allowed,
        }
    }
}

impl IntoRaw<CheckKdaMapContentsRaw> for CheckKdaMapContents {
    fn into_raw(self) -> CheckKdaMapContentsRaw {
        CheckKdaMapContentsRaw {
            contents: self
                .contents
                .into_iter()
                .map(|(k, v)| (k.into_raw(), v.into_raw()))
                .collect(),
            other_kdas_allowed: self.other_kdas_allowed,
        }
    }
}
