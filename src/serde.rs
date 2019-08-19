use crate::DatUrl;
use serde::de::{self, Visitor};
use serde::ser;
use std::fmt;

impl<'a> ser::Serialize for DatUrl<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct DatUrlVisitor;

impl<'de> Visitor<'de> for DatUrlVisitor {
    type Value = DatUrl<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string url")
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match DatUrl::parse(value) {
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
            Ok(ok) => Ok(ok),
        }
    }
}

impl<'de> de::Deserialize<'de> for DatUrl<'de> {
    fn deserialize<D>(deserializer: D) -> Result<DatUrl<'de>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(DatUrlVisitor)
    }
}
