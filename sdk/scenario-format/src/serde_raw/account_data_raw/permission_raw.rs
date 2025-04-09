use serde::{Deserialize, Serialize};

use crate::serde_raw::ValueSubTree;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm: Option<ValueSubTree>,
}
