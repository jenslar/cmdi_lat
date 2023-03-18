use serde::{Deserialize, Serialize};

/// Genre container.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Genres {
    #[serde(rename = "Genre", default)]
    pub values: Vec<Genre>
}

impl Genres {
    pub fn add(&mut self, value: &str) {
        self.values.push(Genre(value.to_owned()))
    }
}

/// Genre value.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Genre(String);

/// SubGenre container.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubGenres {
    #[serde(rename = "SubGenre", default)]
    pub values: Vec<SubGenre>
}

impl SubGenres {
    pub fn add(&mut self, value: &str) {
        self.values.push(SubGenre(value.to_owned()))
    }
}

/// SubGenre value.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubGenre(String);