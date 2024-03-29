use super::*;
use serde::{
    de::{Deserializer, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};
use std::{collections::BTreeMap, fmt};
pub struct CheckKdaMapContentsRaw {
    pub contents: BTreeMap<String, CheckKdaRaw>,
    pub other_kdas_allowed: bool,
}

impl Serialize for CheckKdaMapContentsRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.contents.len()))?;
        for (k, v) in self.contents.iter() {
            map.serialize_entry(k, v)?;
        }
        if self.other_kdas_allowed {
            map.serialize_entry("+", "")?;
        }
        map.end()
    }
}
impl<'de> Deserialize<'de> for CheckKdaMapContentsRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckKdaMapContentsRawVisitor)
    }
}

struct CheckKdaMapContentsRawVisitor;

impl<'de> Visitor<'de> for CheckKdaMapContentsRawVisitor {
    type Value = CheckKdaMapContentsRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("CheckAccountRaw or nothing")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut contents = BTreeMap::<String, CheckKdaRaw>::new();

        // While there are entries remaining in the input, add them
        // into our map.
        let mut other_kdas_allowed = false;

        while let Some((key, value)) = access.next_entry()? {
            if key == "+" {
                other_kdas_allowed = true;
            } else {
                contents.insert(key, value);
            }
        }

        Ok(CheckKdaMapContentsRaw {
            other_kdas_allowed,
            contents,
        })
    }
}
