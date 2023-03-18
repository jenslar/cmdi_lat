use serde::{Deserialize, Serialize};

use crate::Descriptions;

/// Actor language container.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActorLanguages {
    #[serde(rename = "Actor_Language", default)]
    pub languages: Vec<ActorLanguage>
}

impl Default for ActorLanguages {
    fn default() -> Self {
        Self { languages: Vec::default() }
    }
}

impl ActorLanguages {
    pub fn is_empty(&self) -> bool {
        self.languages.is_empty()
    }
}

/// Actor language.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct ActorLanguage {
    /// Language code, incl ISO prefix. E.g. "ISO639-3:kjg".
    #[serde(rename = "Id")]
    pub id: String,
    // pub id: LanguageId, // does this actually work...?
    // #[serde(rename = "Name")]
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MotherTongue")]
    pub mother_tongue: String, // should be controlled_vocabulary::Bool, but get unknown variant
    // #[serde(rename = "MotherTongue")]
    // #[serde(default="BoolOuter::default")]
    // pub mother_tongue: BoolOuter, // creates empty element which is not valid
    // pub mother_tongue: Bool, // should be controlled_vocabulary::Bool, but get unknown variant
    #[serde(rename = "PrimaryLanguage")]
    pub primary_language: String,
    // #[serde(rename = "PrimaryLanguage")]
    // #[serde(default="BoolOuter::default")]
    // pub primary_language: BoolOuter, // creates empty element which is not valid
    // pub primary_language: Bool, // should be controlled_vocabulary::Bool, but get unknown variant
    pub descriptions: Option<Descriptions>,
}

impl Default for ActorLanguage {
    fn default() -> Self {
        Self {
            id: String::default(),
            // id: LanguageId::default(),
            name: String::default(),
            mother_tongue: String::from("Unspecified"),
            primary_language: String::from("Unspecified"),
            // mother_tongue: BoolOuter::default(),
            // primary_language: BoolOuter::default(),
            descriptions: None
        }
    }
}