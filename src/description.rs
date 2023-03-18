use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename="lowercase")]
pub struct Descriptions{
    #[serde(rename = "Description", default)]
    values: Vec<Description>
}

impl Descriptions {
    pub fn from_value(value: &str, iso: Option<&str>) -> Self {
        Self {
            values: vec![Description::new(value, iso)]
        }
    }

    pub fn add(&mut self, description: &Description) {
        self.values.push(description.to_owned())
    }

    pub fn add_from_value(&mut self, value: &str, iso: Option<&str>) {
        self.values.push(Description::new(value, iso))
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn values(&self) -> Vec<Description> {
        self.values.to_owned()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
// pub struct Description(String);
pub struct Description{
    #[serde(rename = "xml:lang")]
    #[serde(skip_serializing_if = "Option::is_none")] // TODO verify that iso attr is not req:ed
    pub language_iso: Option<String>,
    #[serde(rename = "$value")]
    pub value: String
}

impl Description {
    pub fn new(value: &str, iso: Option<&str>) -> Self {
        Self {
            // TODO Use eng as default lang?
            language_iso: iso.map(String::from),
            value: value.to_owned()
        }
    }
}