use crate::serde_raw::ValueSubTree;

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxCallRaw {
    pub from: ValueSubTree,
    pub to: ValueSubTree,

    /// Backwards compatibility only.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_value: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub kda_value: Vec<TxKDARaw>,

    pub function: String,

    #[serde(default)]
    pub arguments: Vec<ValueSubTree>,

    pub gas_limit: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<ValueSubTree>,
}
