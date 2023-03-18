use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Bool {
    #[serde(rename = "$value")]
    #[serde(default="BoolValue::default")]
    value: BoolValue
}

/// CMDI 'boolean' which adds 'Unspecified',
/// and 'Unknown' as valid choices.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BoolValue {
    #[serde(rename="true")]
    True,
    #[serde(rename="false")]
    False,
    Unspecified,
    Unknown
}

impl Default for BoolValue {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl From<&str> for BoolValue {
    fn from(value: &str) -> Self {
        match value {
            "Unspecified" => Self::Unspecified,
            "Unknown" => Self::Unknown,
            "true" => Self::True,
            "false" => Self::False,
            _ => Self::Unknown
        }
    }
}