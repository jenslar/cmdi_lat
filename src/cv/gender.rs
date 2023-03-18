use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Gender {
    #[serde(rename = "$value")]
    #[serde(default="GenderValue::default")]
    value: GenderValue
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GenderValue {
    Male,
    Female,
    #[serde(rename="NAP")]
    Nap,
    Unspecified,
    Unknown
}

impl Default for GenderValue {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl From<&str> for GenderValue {
    fn from(value: &str) -> Self {
        match value {
            "Male" => Self::Male,
            "Female" => Self::Female,
            "NAP" => Self::Nap,
            "Unspecified" => Self::Unspecified,
            "Unknown" => Self::Unknown,
            _ => Self::Unspecified
        }
    }
}