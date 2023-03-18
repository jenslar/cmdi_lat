use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{description::Descriptions, cv::Bool, key::Keys, Defaults};

/// Content language container.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContentLanguages {
    #[serde(rename = "Content_Language", default)]
    pub languages: Vec<ContentLanguage>
}

impl ContentLanguages {
    pub fn add(&mut self, language: &ContentLanguage) {
        self.languages.push(language.to_owned())
    }

    pub fn from_hashmap(languages: &HashMap<usize, ContentLanguage>) -> Self {
        let mut content_languages = Self::default();
        content_languages.languages = languages.iter().map(|(_, l)| l.to_owned()).collect();
        content_languages
    }
}

/// Content language.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentLanguage {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Dominant", default)]
    pub dominant: String,
    // #[serde(rename = "SourceLanguage", default="Bool::default")]
    // #[serde(rename = "SourceLanguage", default="Defaults::unspecified")]
    // #[serde(rename = "SourceLanguage", default)]
    // pub source_language: Bool,
    // #[serde(rename = "TargetLanguage", default="Bool::default")]
    // pub target_language: Bool,
    #[serde(rename = "SourceLanguage", default="Defaults::unspecified")]
    pub source_language: String,
    // #[serde(rename = "TargetLanguage", default="Bool::default")]
    // pub target_language: Bool,
    #[serde(rename = "TargetLanguage", default="Defaults::unspecified")]
    pub target_language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Descriptions>,
    #[serde(rename="Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Keys>
}

impl Default for ContentLanguage {
    fn default() -> Self {
        Self {
            id: String::default(),
            name: String::default(),
            // dominant: Bool::default(),
            dominant: Defaults::unspecified(),
            // source_language: Bool::default(),
            // target_language: Bool::default(),
            source_language: Defaults::unspecified(),
            target_language: Defaults::unspecified(),
            descriptions: None,
            keys: None
        }
    }
}

// /// Actor language container.
// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct ActorLanguages {
//     #[serde(rename = "Actor_Language", default)]
//     pub languages: Vec<ActorLanguage>
// }

// impl Default for ActorLanguages {
//     fn default() -> Self {
//         Self { languages: Vec::default() }
//     }
// }

// impl ActorLanguages {
//     pub fn is_empty(&self) -> bool {
//         self.languages.is_empty()
//     }
// }

// /// Actor language.
// #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
// pub struct ActorLanguage {
//     /// Language code, incl ISO prefix. E.g. "ISO639-3:kjg".
//     #[serde(rename = "Id")]
//     pub id: String,
//     // pub id: LanguageId, // does this actually work...?
//     // #[serde(rename = "Name")]
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "MotherTongue")]
//     pub mother_tongue: String, // should be controlled_vocabulary::Bool, but get unknown variant
//     // #[serde(rename = "MotherTongue")]
//     // #[serde(default="BoolOuter::default")]
//     // pub mother_tongue: BoolOuter, // creates empty element which is not valid
//     // pub mother_tongue: Bool, // should be controlled_vocabulary::Bool, but get unknown variant
//     #[serde(rename = "PrimaryLanguage")]
//     pub primary_language: String,
//     // #[serde(rename = "PrimaryLanguage")]
//     // #[serde(default="BoolOuter::default")]
//     // pub primary_language: BoolOuter, // creates empty element which is not valid
//     // pub primary_language: Bool, // should be controlled_vocabulary::Bool, but get unknown variant
//     pub descriptions: Option<Descriptions>,
// }

// impl Default for ActorLanguage {
//     fn default() -> Self {
//         Self {
//             id: String::default(),
//             // id: LanguageId::default(),
//             name: String::default(),
//             mother_tongue: String::from("Unspecified"),
//             primary_language: String::from("Unspecified"),
//             // mother_tongue: BoolOuter::default(),
//             // primary_language: BoolOuter::default(),
//             descriptions: None
//         }
//     }
// }

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub enum LanguageId {
    ISO639_3(String), // ISO639-3:[a-z]{3}
    Unknown,
    Unspecified
}

impl Default for LanguageId {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl From<&str> for LanguageId {
    fn from(value: &str) -> Self {
        // TODO validate [a-z]{3}
        let code = if !value.starts_with("ISO639-3:") {
            format!("ISO639-3:{value}")
        } else {
            value.to_owned()
        };
        Self::ISO639_3(code)
    }
}