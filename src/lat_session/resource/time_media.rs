use serde::{Deserialize, Serialize};

/// Resource, media file time position.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct TimePosition {
    /// <http://hdl.handle.net/11459/CCR_C-3896_652d9600-df81-c3d1-978a-54e494077f3f>
    #[serde(rename = "Start")]
    start: String,
    /// <http://hdl.handle.net/11459/CCR_C-3897_50871416-220f-1154-2394-b63a27379c08>
    #[serde(rename = "End")]
    end: Option<String>,
}

/// Resource, media file time position.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct CounterPosition {
    /// <http://hdl.handle.net/11459/CCR_C-3896_652d9600-df81-c3d1-978a-54e494077f3f>
    #[serde(rename = "Start")]
    start: String,
    /// <http://hdl.handle.net/11459/CCR_C-3897_50871416-220f-1154-2394-b63a27379c08>
    #[serde(rename = "End")]
    end: Option<String>,
}