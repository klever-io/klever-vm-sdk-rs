use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// KdaBalance  holds information about the kda balance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KdaBalance {
    pub token_identifier: String,
    pub balance: String,
}

// KdaBalanceDataholds the kda balance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdaBalanceData {
    pub kdas: HashMap<String, KdaBalance>,
}

// KdaBalanceResponse holds the kda balance endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdaBalanceResponse {
    pub data: Option<KdaBalanceData>,
    pub error: String,
    pub code: String,
}

// KdaRolesData holds the kda roles data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdaRolesData {
    pub roles: HashMap<String, Vec<String>>,
}

// KdaBalanceResponse holds the kda roles endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdaRolesResponse {
    pub data: Option<KdaRolesData>,
    pub error: String,
    pub code: String,
}
