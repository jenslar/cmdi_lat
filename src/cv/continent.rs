use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Continent {
    #[serde(rename = "$value")]
    #[serde(default="ContinentValue::default")]
    value: ContinentValue
}

/// clarin.eu.cr1.c_1407745712016
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ContinentValue {
    Unknown,
    Unspecified,
    Africa,
    Asia,
    Europe,
    Australia,
    Oceania,
    #[serde(rename="North-America")]
    NorthAmerica,
    #[serde(rename="Middle-America")]
    MiddleAmerica,
    #[serde(rename="South-America")]
    SouthAmerica,
}

impl Default for ContinentValue {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl From<&str> for ContinentValue {
    fn from(value: &str) -> Self {
        match value {
            "Unknown" => Self::Unknown,
            "Unspecified" => Self::Unspecified,
            "Africa" => Self::Africa,
            "Asia" => Self::Asia,
            "Europe" => Self::Europe,
            "Australia" => Self::Australia,
            "Oceania" => Self::Oceania,
            "North-America" => Self::NorthAmerica,
            "Middle-America" => Self::MiddleAmerica,
            "South-America" => Self::SouthAmerica,
            _ => Self::Unspecified
        }
    }
}