use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Involvement {
    #[serde(rename = "$value")]
    #[serde(default="InvolvementValue::default")]
    value: InvolvementValue
}

/// `simpletype-Involvement-clarin.eu.cr1.c_1456409483206`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum InvolvementValue {
    Unknown, // "Unknown"
    Unspecified, // "Unspecified"
    #[serde(rename="elicited")]
    Elicited, // "elicited"
    #[serde(rename="non-elicited")]
    NonElicited, // "non-elicited"
    #[serde(rename="no-observer")]
    NoObserver, // "no-observer"
}

impl Default for InvolvementValue {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl Into<String> for InvolvementValue {
    fn into(self) -> String {
        match self {
            Self::Unknown => "Unknown".to_owned(),
            Self::Unspecified => "Unspecified".to_owned(),
            Self::Elicited => "elicited".to_owned(),
            Self::NonElicited => "non-elicited".to_owned(),
            Self::NoObserver => "no-observer".to_owned(),
        }
    }
}