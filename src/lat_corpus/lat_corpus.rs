use serde::{Deserialize, Serialize};

use crate::{Descriptions, info_link::InfoLink};

use super::corpus_link::CorpusLink;

/// FLAT Collection, a.k.a. `lat-corpus`.
/// - CMDI v1.1 profile [`clarin.eu:cr1:p_1407745712064`](https://catalog.clarin.eu/ds/ComponentRegistry#/?itemId=clarin.eu%3Acr1%3Ap_1407745712064&registrySpace=public)
/// - [CMDI v1.1 schema](https://catalog.clarin.eu/ds/ComponentRegistry/rest/registry/1.1/profiles/clarin.eu:cr1:p_1407745712064/xsd)
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename = "lat-corpus")]
pub struct LatCorpus {
    #[serde(rename = "History")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Descriptions>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub info_links: Option<Vec<InfoLink>>, // works but no tested with lat-corpus containing InfoLink...
    // #[serde(rename = "CorpusLink")]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub corpus_links: Option<Vec<CorpusLink>>,
    #[serde(rename = "InfoLink", default)]
    pub info_links: Vec<InfoLink>, // works but no tested with lat-corpus containing InfoLink...
    #[serde(rename = "CorpusLink", default)]
    pub corpus_links: Vec<CorpusLink>,
}

// impl LatCorpus {
//     pub fn corpus_links(&self) -> &[CorpusLink] {
//         &self.corpus_links
//         // match &self.corpus_links {
//         //     Some(l) => l,
//         //     None => &[]
//         // }
//     }
// }