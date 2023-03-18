use serde::{Deserialize, Serialize};

use crate::{key::{Keys, Key}, description::Descriptions, Defaults};

use super::{communication_context::CommunicationContext, langauge::{ContentLanguages, ContentLanguage}, Genres, SubGenres};


/// Content metadata.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct Content {
    // #[serde(rename = "Genre", default="Defaults::vec_unspecified")]
    // pub genres: Vec<String>, // TODO still only generates single if multiple, wrapper a al Descriptions needed?
    #[serde(rename = "Genre", default)]
    pub genres: Genres, // TODO still only generates single if multiple, wrapper a al Descriptions needed?
    // #[serde(rename = "SubGenre")]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub sub_genre: Option<Vec<String>>,
    // #[serde(rename = "SubGenre", default)]
    // pub sub_genre: Vec<String>,
    #[serde(rename = "SubGenre", default)]
    pub sub_genres: SubGenres,
    #[serde(rename = "Task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    #[serde(rename = "Modalities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<String>, // TODO should be vec? Only as single string elem in xsd though...?
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "CommunicationContext")]
    pub communication_context: CommunicationContext,
    #[serde(rename = "Content_Languages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_languages: Option<ContentLanguages>,
    // #[serde(rename = "Content_Languages", default)]
    // pub content_languages: Vec<ContentLanguage>,
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Keys>,
    #[serde(rename = "descriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Descriptions>,
}

// TODO prefix "add" methods with "add_"
impl Content {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_genre(&mut self, genre: &str) {
        self.genres.add(genre)
    }

    pub fn add_subgenre(&mut self, sub_genre: &str) {
        self.sub_genres.add(sub_genre)
    }

    pub fn add_task(&mut self, task: &str) {
        self.task = Some(task.to_owned())
    }

    pub fn add_modalities(&mut self, modalities: &str) {
        self.modalities = Some(modalities.to_owned())
    }

    pub fn add_subject(&mut self, subject: &str) {
        self.subject = Some(subject.to_owned())
    }

    pub fn communication_context(&mut self, communication_context: CommunicationContext) {
        self.communication_context = communication_context
    }

    // TODO rename to add_language
    pub fn add_contentlanguage(&mut self, language: &ContentLanguage) {
        // self.content_languages.as_mut().map(|cl| cl.languages.push(language));
        // self.content_languages.as_mut().map(
        //     // || Some(ContentLanguages{languages: vec![language]}),
        //     |cl| cl.languages.push(language)
        // );
        // self.content_languages
        //     .map(|l| l.add(&language.to_owned()))
        //     .unwrap_or(Some(ContentLanguages::default().add(language.to_owned())));
        // self.content_languages.push(language.to_owned());
        if let Some(lang) = &mut self.content_languages {
            lang.add(language)
        } else {
            let mut lang = ContentLanguages::default();
            lang.add(language);
            self.content_languages = Some(lang)
        }
    }

    pub fn add_key(&mut self, key: Key) {
        // self.keys.as_mut().map(|k| k.keys.push(key))
        self.keys.as_mut().map(|k| k.add(&key));
    }

    pub fn add_description(&mut self, value: &str, iso: Option<&str>) {
        self.descriptions.as_mut().map(|d| d.add_from_value(value, iso));
    }
}
