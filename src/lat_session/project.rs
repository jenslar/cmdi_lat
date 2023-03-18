use serde::{Deserialize, Serialize};

use crate::{contact::Contact, Descriptions};

/// Location.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Project {
    #[serde(rename = "Name")]
    name: String, // element must exist but still validates if empty, e.g. <Name/>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "Contact")]
    contact: Contact, // 0 .. unbounded -> Vec<Contact>?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Descriptions>,
    // TODO impl InfoLink here: ", clarin.eu:cr1:c_1407745712063, http://hdl.handle.net/11459/CCR_C-2520_9eeedfb4-47d3-ddee-cfcb-99ac634bf1db
}