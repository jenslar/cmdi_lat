use serde::{Deserialize, Serialize};

use crate::{key::Keys, contact::Contact, description::Descriptions};

use super::{ActorLanguages, Age};

/// Actor container.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct Actors{
    #[serde(rename = "Actor", default)]
    pub actors: Vec<Actor>
}

/// Actor element.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct Actor {
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "FullName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "FamilySocialRole")]
    pub family_social_role: String,
    #[serde(rename = "EthnicGroup")]
    pub ethnic_group: String,
    #[serde(rename = "BirthDate")]
    pub birth_date: String,
    #[serde(rename = "Sex")]
    pub sex: String, // should be enum Male, Female, NAP, Unspecified, Unknown
    #[serde(rename = "Education")]
    pub education: String,
    #[serde(rename = "Anonymized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymized: Option<String>,
    pub age: Age,
    pub contact: Option<Contact>,
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Keys>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Descriptions>,
    #[serde(rename = "Actor_Languages", default)]
    pub actor_languages: Option<ActorLanguages>,
}

impl Default for Actor {
    fn default() -> Self {
        Self{
            role: String::default(),
            name: String::default(),
            full_name: None,
            code: None,
            family_social_role: String::from("Unspecified"),
            ethnic_group: String::from("Unspecified"),
            birth_date: String::from("Unspecified"),
            sex: String::from("Unspecified"), // enum Male, Female, NAP, Unspecified, Unknown
            education: String::from("Unspecified"),
            anonymized: None,
            age: Age::default(),
            contact: None,
            keys: None,
            descriptions: None,
            actor_languages: None,
        }
    }
}