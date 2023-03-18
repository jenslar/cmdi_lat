use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename="lowercase")]
pub struct CorpusLinks{
    #[serde(rename = "CorpusLink", default)]
    pub corpus_links: Vec<CorpusLink>
}

// #[derive(Debug, Clone, Default, Deserialize, Serialize)]
// pub struct Name{
//     #[serde(rename="$value")]
//     value: String
// }

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CorpusLink {
    /// Attribute. Reference ID for corresponding `ResourceProxy`.
    /// Must be a valid NCName, i.e. can not start with a digit.
    #[serde(rename = "@ref")]
    pub corpus_ref: String,
    /// Child node. `Name`/description field,
    /// similar to CMDI `Name`/`Title` fields.
    #[serde(rename = "Name")]
    pub name: String
}

// #[derive(Debug, Default, Clone, Deserialize, Serialize)]
// pub struct Name{
//     #[serde(rename = "$value")]
//     value: String
// }