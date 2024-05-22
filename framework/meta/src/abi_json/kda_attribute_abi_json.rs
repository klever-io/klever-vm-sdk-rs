use std::collections::BTreeMap;

use klever_sc::abi::KdaAttributeAbi;
use serde::{Deserialize, Serialize};
use crate::abi_json::kda_attribute_json::KdaAttributeJson;
use crate::abi_json::{convert_type_descriptions_to_json, TypeDescriptionJson};

/// Represents an entire KDA attribute ABI file. The type descriptions only show up here.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KdaAttributeAbiJson {
    pub kda_attribute: KdaAttributeJson,

    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub types: BTreeMap<String, TypeDescriptionJson>,
}

impl KdaAttributeAbiJson {
    pub fn new(attr: &KdaAttributeAbi) -> Self {
        KdaAttributeAbiJson {
            kda_attribute: KdaAttributeJson::from(attr),
            types: convert_type_descriptions_to_json(&attr.type_descriptions),
        }
    }
}