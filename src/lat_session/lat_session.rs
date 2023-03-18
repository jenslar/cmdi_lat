use serde::{Deserialize, Serialize};

use super::{
    location::Location,
    content::Content,
    actor::Actors,
    resource::resource::{LatResources, LatResource},
    Project
};
use crate::{key::Keys, description::Descriptions};
// use crate::resources::Resources;

/// FLAT Bundle, a.k.a. `lat-session`.
/// - CMDI v1.1 profile [`clarin.eu:cr1:p_1407745712035`](https://catalog.clarin.eu/ds/ComponentRegistry#/?itemId=clarin.eu%3Acr1%3Ap_1407745712035&registrySpace=public)
/// - [CMDI v1.1 schema](https://catalog.clarin.eu/ds/ComponentRegistry/rest/registry/1.1/profiles/clarin.eu:cr1:p_1407745712035/xsd)
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename = "lat-session")]
pub struct LatSession {
    #[serde(rename = "History")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")] // or serialize anyawy?
    pub title: Option<String>,
    #[serde(rename = "Date")]
    pub date: String, // TODO implement proper date object de/serialization + enum (ISO8061 | Unknown | Unspecified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Descriptions>,
    // TODO implement InfoLink after descriptions
    #[serde(rename = "Location")]
    pub location: Location,
    #[serde(rename = "Project")]
    pub project: Project, // 0 .. unbounded according to XSD?
    #[serde(rename="Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Keys>,
    #[serde(rename = "Content")]
    pub content: Content,
    #[serde(rename = "Actors")]
    pub actors: Actors,
    #[serde(rename = "Resources")]
    pub resources: LatResources,
    // TODO implement References
    // #[serde(rename = "References")]
    // pub references: References,
}

// impl Default for LatSession {
//     fn default() -> Self {
//         Self {
//             history: None,
//             name: String::default(),
//             title: None,
//             date: String::from("Unspecified"), // TODO implement proper date object de/serialization + 
//             descriptions: None,
//             location: Location,
//             project: Project,
//             keys: Option<Keys>,
//             content: Content,
//             actors: Actors,
//             resources: Resources,
//         }
//     }
// }
