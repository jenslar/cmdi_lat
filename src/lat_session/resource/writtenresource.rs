use serde::{Deserialize, Serialize};

use crate::{key::Key, description::Descriptions};

use super::access::Access;

/// Resource, written.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WrittenResource {
    #[serde(rename = "@ref")]
    resource_ref: Option<String>,
    #[serde(rename = "@mediaRef")]
    media_ref: Option<String>,
    #[serde(rename = "Date")]
    /// <http://hdl.handle.net/11459/CCR_C-2510_2402e609-046a-dfbf-c2d7-5a2f1ae6dc86>
    date: String,
    #[serde(rename = "Type")]
    /// <http://hdl.handle.net/11459/CCR_C-3900_8cbd76e3-556e-8271-1baf-b2170c7017ab>
    media_type: String,
    #[serde(rename = "SubType")]
    /// <http://hdl.handle.net/11459/CCR_C-3901_af291d73-7e01-a844-b88e-0d3f0c95bd84>
    media_sub_type: Option<String>,
    #[serde(rename = "Format")]
    /// <http://hdl.handle.net/11459/CCR_C-2465_4444eb51-7cf7-0ff7-7687-7f741f3a4f84>
    format: String,
    #[serde(rename = "Size")]
    /// <http://hdl.handle.net/11459/CCR_C-2580_6dfe4e09-1c61-9b24-98ad-16bb867860fe>
    size: String,
    #[serde(rename = "Derivation")]
    /// <http://hdl.handle.net/11459/CCR_C-2518_ea48054d-f23f-c493-bcc0-067561b87c67>
    derivation: Option<String>,
    #[serde(rename = "CharacterEncoding")]
    character_encoding: Option<String>,
    #[serde(rename = "ContentEncoding")]
    content_encoding: Option<String>,
    #[serde(rename = "LanguageId")]
    language_id: Option<String>,
    #[serde(rename = "Anonymized")]
    anonymized: Option<String>,
    validation: Option<Validation>,
    // #[serde(rename = "PascalCase")]
    descriptions: Option<Descriptions>,
    #[serde(rename = "Access")]
    access: Access,
    #[serde(rename = "Keys", default)]
    keys: Vec<Key>,
}

/// Resource validation, written.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Validation {
    #[serde(rename = "Type")]
    /// <http://hdl.handle.net/11459/CCR_C-2587_026dcaa6-8ece-3364-8492-6479e70f66de>
    validation_type: String,
    #[serde(rename = "Methodology")]
    /// <http://hdl.handle.net/11459/CCR_C-2586_75098d25-a517-983f-817c-2b05c5ce361a>
    methodology: String,
    #[serde(rename = "Level")]
    /// <http://hdl.handle.net/11459/CCR_C-2585_0fcf8ea5-a273-d077-86e9-5e8f1c50fe72>
    level: String,
    // #[serde(rename = "PascalCase")]
    descriptions: Descriptions,
}