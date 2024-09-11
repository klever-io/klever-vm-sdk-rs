use klever_sc::abi::{KdaAttributeAbi, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KdaAttributeJson {
    pub ticker: String,
    #[serde(rename = "type")]
    pub ty: TypeName,
}

impl From<&KdaAttributeAbi> for KdaAttributeJson {
    fn from(attr: &KdaAttributeAbi) -> Self {
        KdaAttributeJson {
            ticker: attr.ticker.to_owned(),
            ty: attr.ty.clone(),
        }
    }
}
