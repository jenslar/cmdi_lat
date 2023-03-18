//! Key/s were custom fields in IMDI, essentially key/value stores, but may be ignored and/or discarded by FLAT CMDI.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename="lowercase")]
pub struct Keys{
    #[serde(rename = "Key", default)]
    pub keys: Vec<Key>
}

impl Keys {
    pub fn add(&mut self, key: &Key) {
        self.keys.push(key.to_owned())
    }
}

/// Key. Only to ensure backwards compatibility to IMDI metadata. Please only use it for this purpose.
/// 
/// <http://hdl.handle.net/11459/CCR_C-4174_8c1a1567-a86d-50d8-e208-e030f2688c98>
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Key {
    #[serde(rename="Name")]
    key_name: Option<String>, // TODO deserialization can't find attribute here, issue with $value in parallel?
    #[serde(rename="Link")]
    key_link: Option<String>, // TODO deserialization can't find attribute here, issue with $value in parallel?
    #[serde(rename="Type")]
    key_type: Option<String>, // TODO deserialization can't find attribute here, issue with $value in parallel?
    #[serde(rename="$value")]
    value: String
}