use serde::{Deserialize, Serialize};

use crate::serde_raw::{CheckBytesValueRaw, CheckKdaInstancesRaw};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckKdaDataRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "CheckKdaInstancesRaw::is_unspecified")]
    pub instances: CheckKdaInstancesRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub last_nonce: CheckBytesValueRaw,

    /// Currently not actually checked anywhere.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub frozen: CheckBytesValueRaw,
}
