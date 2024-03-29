use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Default)]
pub enum CheckKdaMapRaw {
    #[default]
    Unspecified,
    Star,
    Equal(CheckKdaMapContentsRaw),
}

impl CheckKdaMapRaw {
    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckKdaMapRaw::Unspecified)
    }

    pub fn is_star(&self) -> bool {
        matches!(self, CheckKdaMapRaw::Star)
    }
}

impl Serialize for CheckKdaMapRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckKdaMapRaw::Unspecified => serializer.serialize_str(""),
            CheckKdaMapRaw::Star => serializer.serialize_str("*"),
            CheckKdaMapRaw::Equal(m) => m.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CheckKdaMapRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckKdaMapRawVisitor)
    }
}
struct CheckKdaMapRawVisitor;

impl<'de> Visitor<'de> for CheckKdaMapRawVisitor {
    type Value = CheckKdaMapRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of log check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckKdaMapRaw::Star)
        } else {
            Err(de::Error::custom("only '*' allowed as logs string value"))
        }
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        Ok(CheckKdaMapRaw::Equal(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }
}
