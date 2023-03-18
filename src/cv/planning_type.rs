use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct PlanningType {
    #[serde(rename = "$value")]
    #[serde(default="PlanningTypeValue::default")]
    value: PlanningTypeValue
}

/// `simpletype-PlanningType-clarin.eu.cr1.c_1456409483206`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PlanningTypeValue {
    Unknown, // "Unknown"
    Unspecified, // "Unspecified"
    #[serde(rename="spontaneous")]
    Spontaneous, // "spontaneous"
    #[serde(rename="semi-spontaneous")]
    SemiSpontaneous, // "semi-spontaneous"
    #[serde(rename="planned")]
    Planned, // "planned"
}

impl Default for PlanningTypeValue {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl Into<String> for PlanningTypeValue {
    fn into(self) -> String {
        match self {
            Self::Unknown => "Unknown".to_owned(),
            Self::Unspecified => "Unspecified".to_owned(),
            Self::Spontaneous => "spontaneous".to_owned(),
            Self::SemiSpontaneous => "semi-spontaneous".to_owned(),
            Self::Planned => "planned".to_owned(),
        }
    }
}