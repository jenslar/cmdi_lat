use serde::{Deserialize, Serialize};

use crate::Description;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InfoLink{
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Description>
}