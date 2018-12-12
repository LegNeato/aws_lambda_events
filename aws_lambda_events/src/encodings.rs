use super::custom_serde::*;
use chrono::{DateTime, Utc};

/// Binary data encoded in base64.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Base64Data(
    #[serde(deserialize_with = "deserialize_base64")]
    #[serde(serialize_with = "serialize_base64")]
    pub Vec<u8>,
);

/// Timestamp with millisecond precision.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MillisecondTimestamp(
    #[serde(deserialize_with = "deserialize_milliseconds")]
    #[serde(serialize_with = "serialize_milliseconds")]
    pub DateTime<Utc>,
);

/// Timestamp with second precision.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SecondTimestamp(
    #[serde(deserialize_with = "deserialize_seconds")]
    #[serde(serialize_with = "serialize_seconds")]
    pub DateTime<Utc>,
);
