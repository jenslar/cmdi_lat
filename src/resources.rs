use std::path::{PathBuf, Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CmdResources {
    // #[serde(rename = "CorpusLink")]
    // pub corpus_links: Option<Vec<CorpusLink>>,
    pub resource_proxy_list: CmdResourceProxyList,
    pub journal_file_proxy_list: CmdJournalFileProxyList,
    pub resource_relation_list: CmdResourceRelationList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmdResourceProxyList{
    #[serde(rename = "ResourceProxy", default)]
    pub resource_proxies: Vec<CmdResourceProxy>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CmdResourceProxy {
    // #[serde(default)]
    #[serde(rename="@id")]
    pub id: String,
    #[serde(rename = "ResourceType")]
    pub resource_type: CmdResourceType,
    #[serde(rename = "ResourceRef")]
    pub resource_ref: CmdResourceRef,
}

impl CmdResourceProxy {
    pub fn is_resource(&self) -> bool {
        self.resource_type() == "Resource"
    }

    pub fn is_metadata(&self) -> bool {
        self.resource_type() == "Metadata"
    }

    /// FLAT Islandora PID value from attribute `lat:flatURI`.
    /// Possibly contains further info encoded,
    /// such as whether `CMD` or not, and a version time stamp.
    /// e.g. `lat:10050_0f473b55_928f_4c67_9aea_c35f8e9694c8#CMD@2023-01-20T11:34:01.027Z`
    pub fn flat_uri(&self) -> Option<&str> {
        self.resource_ref.lat_flat_uri.as_deref()
    }

    /// Returns path to resource specified by `lat:flatURI`.
    /// Note that for an ingested Islandora object this
    /// is never a path, but a PID.
    pub fn flat_uri_path(&self, root: Option<&Path>) -> Option<PathBuf> {
        let root = root.unwrap_or(Path::new("."));
        self.flat_uri().map(|uri| root.join(PathBuf::from(uri)))
    }

    /// Returns path to resource specified by `lat:localURI` as `&str`.
    /// Not present for ingested FLAT Islandora objects.
    pub fn local_uri(&self) -> Option<&str> {
        self.resource_ref.lat_local_uri.as_deref()
    }

    /// Returns path to resource specified by `lat:localURI`.
    /// Not present for ingested FLAT Islandora objects.
    pub fn local_uri_path(&self, root: Option<&Path>) -> Option<PathBuf> {
        let root = root.unwrap_or(Path::new("."));
        self.local_uri().map(|uri| root.join(PathBuf::from(uri)))
    }
    
    /// FLAT Islandora PID.
    pub fn pid(&self) -> Option<String> {
        if let Some(uri) = &self.resource_ref.lat_flat_uri {
            // split e.g. `lat:10050_0f473b55_928f_4c67_9aea_c35f8e9694c8#CMD@2023-01-20T11:34:01.027Z`
            uri.split_once('#')
                .map(|spl| spl.0.to_owned())
                .or_else(|| Some(uri.to_owned()))
            // match uri.split_once('#') {
            //     Some(spl) => Some(spl.0.to_owned()),
            //     None => Some(uri.to_owned())
            // }
        } else {
            None
        }
    }

    pub fn resource_type(&self) -> &str {
        self.resource_type.value.as_str()
    }
}

enum ResourceType {
    Resource,
    Metadata,
    LandingPage
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CmdResourceType {
    #[serde(rename="@mimetype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
    // TODO should be enum: Metadata, Resource
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CmdResourceRef {
    #[serde(rename = "@localURI")] // lat:localURI, not listed in lat-session/corpus.xsd
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat_local_uri: Option<String>,
    #[serde(rename = "@flatURI")] // lat:flatURI, not listed in lat-session/corpus.xsd
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat_flat_uri: Option<String>,
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmdJournalFileProxyList{
    #[serde(rename = "JournalFileProxy", default)]
    pub journal_file_proxies: Vec<CmdJournalFileProxy>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CmdJournalFileProxy {} // TODO implement

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmdResourceRelationList{
    #[serde(rename = "ResourceRelation", default)]
    pub resource_relation: Vec<CmdResourceRelation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CmdResourceRelation {} // TODO implement
