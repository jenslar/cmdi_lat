use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct EventStructure {
    #[serde(rename = "$value")]
    #[serde(default="EventStructureValue::default")]
    value: EventStructureValue
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EventStructureValue {
    Unknown, // "Unknown"
    Unspecified, // "Unspecified"
    Monologue, // "Monologue"
    Dialogue, // "Dialogue"
    Multilogue, // "Multilogue"
    #[serde(rename="Not a natural format")]
    NotANaturalFormat, // "Not a natural format"
    Conversation, // "Conversation"
}

impl Default for EventStructureValue {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl Into<String> for EventStructureValue {
    fn into(self) -> String {
        match self {
            Self::Unknown => "Unknown".to_owned(),
            Self::Unspecified => "Unspecified".to_owned(),
            Self::Monologue => "Monologue".to_owned(),
            Self::Dialogue => "Dialogue".to_owned(),
            Self::Multilogue => "Multilogue".to_owned(),
            Self::NotANaturalFormat => "Not a natural format".to_owned(),
            Self::Conversation => "Conversation".to_owned(),
        }
    }
}