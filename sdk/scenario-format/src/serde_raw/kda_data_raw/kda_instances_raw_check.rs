use super::*;
use serde::{
    de::{self, Deserializer, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, Serializer},
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Default)]
pub enum CheckKdaInstancesRaw {
    #[default]
    Unspecified,
    Star,
    Equal(Vec<CheckKdaInstanceRaw>),
}

impl CheckKdaInstancesRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckKdaInstancesRaw::Star)
    }

    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckKdaInstancesRaw::Unspecified)
    }
}

impl Serialize for CheckKdaInstancesRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckKdaInstancesRaw::Unspecified => {
                let map = serializer.serialize_map(Some(0))?;
                map.end()
            },
            CheckKdaInstancesRaw::Star => serializer.serialize_str("*"),
            CheckKdaInstancesRaw::Equal(m) => {
                let mut map = serializer.serialize_seq(Some(m.len()))?;
                for v in m {
                    map.serialize_element(v)?;
                }
                map.end()
            },
        }
    }
}

struct CheckKdaInstancesRawVisitor;

impl<'de> Visitor<'de> for CheckKdaInstancesRawVisitor {
    type Value = CheckKdaInstancesRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of an KDA instances list")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckKdaInstancesRaw::Star)
        } else {
            Err(de::Error::custom("only '*' allowed as KDA instances value"))
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::<CheckKdaInstanceRaw>::new();

        while let Some(item) = seq.next_element()? {
            list.push(item);
        }

        Ok(CheckKdaInstancesRaw::Equal(list))
    }
}

impl<'de> Deserialize<'de> for CheckKdaInstancesRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckKdaInstancesRawVisitor)
    }
}
