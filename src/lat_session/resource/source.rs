use serde::{Deserialize, Serialize};

use crate::description::Descriptions;

use super::{time_media::{TimePosition, CounterPosition}, access::Access};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Source {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Format")]
    format: String,
    #[serde(rename = "Quality")]
    quality: String,
    counter_position: Option<CounterPosition>,
    time_position: Option<TimePosition>,
    access: Access,
    #[serde(rename = "descriptions")]
    descriptions: Option<Descriptions>,
}