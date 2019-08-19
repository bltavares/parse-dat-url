use crate::DatUrl;
use serde::de;
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

impl<'de> de::Visitor<'de> for DatUrlVisitor {
    type Value = DatUrl<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string url")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DatUrl::parse(value)
            .map_err(de::Error::custom)
            .map(DatUrl::into_owned)
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DatUrl::parse(value).map_err(de::Error::custom)
    }
}

impl<'de: 'a, 'a> de::Deserialize<'de> for DatUrl<'a> {
    fn deserialize<D>(deserializer: D) -> Result<DatUrl<'a>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(DatUrlVisitor)
    }
}
