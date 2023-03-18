use serde::{Deserialize, Serialize};

use crate::{LatSession, LatCorpus};

// TODO only used for lat-session right now, change to both corpus+session when quick-xml bug is fixed
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Components")]
pub struct Components {
    #[serde(rename = "$value")]
    pub component_type: ComponentType
}

// TODO enums generates douple tags, possible fix in next release: https://github.com/tafia/quick-xml/issues/346 + 490? (merged into master, but unreleased as of 221101)
#[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(untagged)]
pub enum ComponentType {
    #[serde(rename = "lat-session")]
    LatSession(LatSession),
    #[serde(rename = "lat-corpus")]
    LatCorpus(LatCorpus),
}

impl Default for ComponentType {
    fn default() -> Self {
        Self::LatSession(LatSession::default())
    }
}
