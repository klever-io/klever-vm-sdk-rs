use crate::{
    scenario::model::U64Value,
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::PermissionRaw,
    },
    scenario_model::AddressValue,
};

#[derive(Debug, Default, Clone)]
pub struct Permission {
    pub address: Option<AddressValue>,
    pub perm: Option<U64Value>,
}

impl InterpretableFrom<PermissionRaw> for Permission {
    fn interpret_from(from: PermissionRaw, context: &InterpreterContext) -> Self {
        Permission {
            address: from
                .address
                .map(|v| AddressValue::interpret_from(v, context)),
            perm: from.perm.map(|n| U64Value::interpret_from(n, context)),
        }
    }
}

impl IntoRaw<PermissionRaw> for Permission {
    fn into_raw(self) -> PermissionRaw {
        PermissionRaw {
            address: self.address.map(|v| v.original),
            perm: self.perm.map(|n| n.original),
        }
    }
}
