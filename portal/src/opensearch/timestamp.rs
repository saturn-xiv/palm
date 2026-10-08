use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%dT%H:%M:%S.%fZ";

pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // %f pads fractional seconds to 9 digits (nanoseconds) by default in chrono
    let s = date.format(FORMAT).to_string();
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
}
