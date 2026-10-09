//! Serde support using the RFC 3339 / RFC 9557 representation.

use core::fmt;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

use crate::datetime::DateTime;

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

struct DateTimeVisitor;

impl Visitor<'_> for DateTimeVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an RFC 3339 or RFC 9557 datetime string")
    }

    fn visit_str<E>(self, value: &str) -> Result<DateTime, E>
    where
        E: de::Error,
    {
        value.parse().map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DateTimeVisitor)
    }
}
