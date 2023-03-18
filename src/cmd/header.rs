use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PascalCase")]
pub struct Header {
    #[serde(rename = "MdCreator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md_creator: Option<String>,
    #[serde(rename = "MdCreationDate")]
    #[serde(default="Header::today")]
    pub md_creation_date: String,
    #[serde(rename = "MdSelfLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md_self_link: Option<String>,
    #[serde(rename = "MdProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md_profile: Option<String>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            md_creator: None,
            md_creation_date: Self::today(),
            md_self_link: None,
            md_profile: Some(Self::session()),
        }
    }
}

impl Header {
    /// Returns current date as string.
    pub fn today() -> String {
        // time::OffsetDateTime::now_utc().to_string()
        time::OffsetDateTime::now_utc().date().to_string()
    }

    /// Returns CMDI profile ID for `lat-corpus`
    /// as a string (`clarin.eu:cr1:p_1407745712064`).
    pub fn corpus() -> String {
        "clarin.eu:cr1:p_1407745712064".to_owned()
    }
    
    /// Returns CMDI profile ID for `lat-session`.
    /// as a string (`clarin.eu:cr1:p_1407745712035`).
    pub fn session() -> String {
        "clarin.eu:cr1:p_1407745712035".to_owned()
    }
}

// #[derive(Debug, Clone, Default, Deserialize, Serialize)]
// pub struct MdCreator(String);

// #[derive(Debug, Clone, Default, Deserialize, Serialize)]
// pub struct MdCreationDate(String);

// impl MdCreationDate {
//     /// Returns `MdCreationDate` with current date as string.
//     pub fn today() -> Self {
//         Self(time::OffsetDateTime::now_utc().to_string()) // TODO change to proper de/serialize to string with time object
//     }
// }

// #[derive(Debug, Clone, Default, Deserialize, Serialize)]
// pub struct MdSelfLink(String);

// #[derive(Debug, Clone, Default, Deserialize, Serialize)]
// pub struct MdProfile(String);

// impl MdProfile {
//     pub fn corpus() -> Self {
//         Self("clarin.eu:cr1:p_1407745712064".to_owned())
//     }

//     pub fn session() -> Self {
//         Self("clarin.eu:cr1:p_1407745712035".to_owned())
//     }
// }
