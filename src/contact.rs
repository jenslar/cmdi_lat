use serde::{Deserialize, Serialize};

/// Location.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Contact {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(rename = "Organisation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    organisation: Option<String>,
}