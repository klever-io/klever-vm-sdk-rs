use crate::{
    scenario::model::{BigUintValue, BytesValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::KdaInstanceRaw,
    },
};

#[derive(Debug, Default, Clone)]
pub struct KdaInstance {
    pub nonce: Option<U64Value>,
    pub balance: Option<BigUintValue>,
    pub creator: Option<BytesValue>,
    pub royalties: Option<U64Value>,
    pub hash: Option<BytesValue>,
    pub uri: Vec<BytesValue>,
    pub attributes: Option<BytesValue>,
    pub can_burn: Option<bool>,
}

impl KdaInstance {
    pub fn is_simple_fungible(&self) -> bool {
        let is_fungible = if let Some(nonce) = &self.nonce {
            nonce.value == 0
        } else {
            true
        };

        is_fungible
            && self.creator.is_none()
            && self.royalties.is_none()
            && self.hash.is_none()
            && self.uri.is_empty()
            && self.attributes.is_none()
    }
}

impl InterpretableFrom<KdaInstanceRaw> for KdaInstance {
    fn interpret_from(from: KdaInstanceRaw, context: &InterpreterContext) -> Self {
        KdaInstance {
            nonce: from.nonce.map(|n| U64Value::interpret_from(n, context)),
            balance: from
                .balance
                .map(|b| BigUintValue::interpret_from(b, context)),
            creator: from.creator.map(|b| BytesValue::interpret_from(b, context)),
            royalties: from.royalties.map(|b| U64Value::interpret_from(b, context)),
            hash: from.hash.map(|b| BytesValue::interpret_from(b, context)),
            uri: from
                .uri
                .into_iter()
                .map(|b| BytesValue::interpret_from(b, context))
                .collect(),
            attributes: from
                .attributes
                .map(|b| BytesValue::interpret_from(b, context)),
            can_burn: from.can_burn.map(|b| b.into()),
        }
    }
}

impl IntoRaw<KdaInstanceRaw> for KdaInstance {
    fn into_raw(self) -> KdaInstanceRaw {
        KdaInstanceRaw {
            nonce: self.nonce.map(|n| n.original),
            balance: self.balance.map(|n| n.original),
            creator: self.creator.map(|n| n.original),
            royalties: self.royalties.map(|n| n.original),
            hash: self.hash.map(|n| n.original),
            uri: self.uri.into_iter().map(|b| b.original).collect(),
            attributes: self.attributes.map(|n| n.original),
            can_burn: self.can_burn.map(| b | b.into()),
        }
    }
}
