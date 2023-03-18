use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct Age {
    #[serde(rename="EstimatedAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_age: Option<String>,
    #[serde(rename="ExactAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_age: Option<ExactAge>,
    #[serde(rename="AgeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<AgeRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeStamp {
    pub years: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct ExactAge(TimeStamp);

impl ExactAge {
    pub fn years(&self) -> u32 {
        self.0.years
    }

    pub fn months(&self) -> u32 {
        self.0.months.unwrap_or_default()
    }

    pub fn days(&self) -> u32 {
        self.0.days.unwrap_or_default()
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct AgeRange{
    #[serde(rename="MinimumAge")]
    minimum_age: TimeStamp,
    #[serde(rename="MaximumAge")]
    maximum_age: TimeStamp
}