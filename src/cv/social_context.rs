use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct SocialContext {
    #[serde(rename = "$value")]
    #[serde(default="SocialContextValue::default")]
    value: SocialContextValue
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SocialContextValue {
    Unknown, // "Unknown"
    Unspecified, // "Unspecified"
    Family, // "Family"
    Private, // "Private"
    Public, // "Public"
    #[serde(rename="Controlled environment")]
    ControlledEnvironment, // "Controlled environment"
    #[serde(rename="Public (school)")]
    PublicSchool, // "Public (school)"
    Community, // "Community"
}

impl Default for SocialContextValue {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl Into<String> for SocialContextValue {
    fn into(self) -> String {
        match self {
            Self::Unknown => "Unknown".to_owned(),
            Self::Unspecified => "Unspecified".to_owned(),
            Self::Family => "Family".to_owned(),
            Self::Private => "Private".to_owned(),
            Self::Public => "Public".to_owned(),
            Self::ControlledEnvironment => "Controlled environment".to_owned(),
            Self::PublicSchool => "Public (school)".to_owned(),
            Self::Community => "Community".to_owned(),
        }
    }
}