use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Channel {
    #[serde(rename = "$value")]
    #[serde(default="ChannelValue::default")]
    value: ChannelValue
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ChannelValue {
    Unknown,
    Unspecified,
    #[serde(rename="Face to Face")] // old imdi that have 'Face to face' (lower case 2nd face...)
    FaceToFace,
    #[serde(rename="Experimental setting")]
    ExperimentalSetting,
    Broadcasting,
    Telephone,
    #[serde(rename="wizard-of-oz")]
    WizardOfOz,
    #[serde(rename="Human-machine dialogue")]
    HumanMachineDialogue,
    Other,
}

impl Default for ChannelValue {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl Into<String> for ChannelValue {
    fn into(self) -> String {
        match self {
            Self::Unknown => "Unknown".to_owned(),
            Self::Unspecified => "Unspecified".to_owned(),
            Self::FaceToFace => "Face to Face".to_owned(),
            Self::ExperimentalSetting => "Experimental setting".to_owned(),
            Self::Broadcasting => "Broadcasting".to_owned(),
            Self::Telephone => "Telephone".to_owned(),
            Self::WizardOfOz => "wizard-of-oz".to_owned(),
            Self::HumanMachineDialogue => "Human-machine dialogue".to_owned(),
            Self::Other => "Other".to_owned(),
        }
    }
}