use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Interactivity {
    #[serde(rename = "$value")]
    #[serde(default="InteractivityValue::default")]
    value: InteractivityValue
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum InteractivityValue {
    Unknown, // Unknown,
    Unspecified, // Unspecified,
    #[serde(rename="interactive")]
    Interactive, // interactive,
    #[serde(rename="non-interactive")]
    NonInteractive, // non-interactive,
    #[serde(rename="semi-interactive")]
    SemiInteractive, // semi-interactive,
}

impl Default for InteractivityValue {
    fn default() -> Self {
        Self::Unspecified
    }
}

// impl Into<String> for InteractivityValue {
//     fn into(self) -> String {
//         match self {
//             Self::Unknown => "Unknown".to_owned(),
//             Self::Unspecified => "Unspecified".to_owned(),
//             Self::Interactive => "interactive".to_owned(),
//             Self::NonInteractive => "non-interactive".to_owned(),
//             Self::SemiInteractive => "semi-interactive".to_owned(),
//         }
//     }
// }
impl From<String> for InteractivityValue {
    fn from(string: String) -> Self {
        match string.as_str() {
            "Unknown" => Self::Unknown,
            "Unspecified" => Self::Unspecified,
            "interactive" => Self::Interactive,
            "non-interactive" => Self::NonInteractive,
            "semi-interactive" => Self::SemiInteractive,
            _ => Self::default()
        }
    }
}