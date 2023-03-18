use serde::{Deserialize, Serialize};

use super::{access::Access, time_media::TimePosition};

/// Resource, media file.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct MediaFile {
    // #[serde(rename = "ref")] // TODO 220825 ignore for now, since collides with ugly string replace for enum bug fix
    // r#ref: String,
    #[serde(rename = "Type")]
    /// <http://hdl.handle.net/11459/CCR_C-2570_6f596a6d-4d5c-f336-d971-e61a310e2c8c>
    media_type: String,
    #[serde(rename = "Format")]
    /// <http://hdl.handle.net/11459/CCR_C-2571_2be2e583-e5af-34c2-3673-93359ec1f7df>
    format: String,
    #[serde(rename = "Size")]
    /// <http://hdl.handle.net/11459/CCR_C-2580_6dfe4e09-1c61-9b24-98ad-16bb867860fe>
    size: String,
    #[serde(rename = "Quality")]
    /// <http://hdl.handle.net/11459/CCR_C-2574_85e2b550-e877-599a-fb82-76812896f9da>
    quality: String,
    #[serde(rename = "RecordingConditions")]
    /// <http://hdl.handle.net/11459/CCR_C-2566_5a4ee887-bc58-38ee-9b1e-a06f1916d63c>
    recording_conditions: Option<String>,
    time_position: Option<TimePosition>,
    access: Option<Access>,
}

// /// Resource, media file time position.
// #[derive(Debug, Clone, Default, Deserialize, Serialize)]
// #[serde(rename_all="PascalCase")]
// pub struct TimePosition {
//     /// <http://hdl.handle.net/11459/CCR_C-3896_652d9600-df81-c3d1-978a-54e494077f3f>
//     start: String,
//     /// <http://hdl.handle.net/11459/CCR_C-3897_50871416-220f-1154-2394-b63a27379c08>
//     end: Option<String>,
// }

