use serde::{Deserialize, Serialize};

use crate::contact::Contact;

/// Access.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Access {
    /// - free
    /// - free for academic use
    /// - restricted use (DEFAULT)
    /// - request required
    /// - user licence required
    /// - registration required
    /// - unknown
    /// 
    /// <http://hdl.handle.net/11459/CCR_C-2453_1f0c3ea5-7966-ae11-d3c6-448424d4e6e8>
    #[serde(rename = "Availability")]
    availability: String,
    #[serde(rename = "Date")]
    // date returns ' missing field `Date` if 'String' for CMDI with "Unspecified"...? because the other Access fields are Option<>???
    date: String, // TODO make DateTime object, should quite possibly be ISO8061, but can't find cmd:complextype-Date-clarin.eu.cr1.c_1407745712053
    #[serde(rename = "Owner")]
    owner: String,
    #[serde(rename = "Publisher")]
    publisher: String,
    #[serde(rename = "Contact")]
    contact: Contact,
}