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

impl CmdResources {
    pub fn add_resource_proxy(&mut self, resource: &CmdResourceProxy) {
        self.resource_proxy_list.add(resource)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmdResourceProxyList{
    #[serde(rename = "ResourceProxy", default)]
    pub resource_proxies: Vec<CmdResourceProxy>,
}

impl CmdResourceProxyList {
    pub fn add(&mut self, resource: &CmdResourceProxy) {
        self.resource_proxies.push(resource.to_owned())
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CmdResourceProxy {
    // #[serde(default)]
    #[serde(rename="@id")]
    pub id: String,
    /// One of:
    /// - CMDI metadata (`Metadata`)
    /// - Data file (`Resource`)
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

    /// New resource proxy. Links data (`Resource`)
    /// or more CMDI-files (`Metadata`).
    /// `mimetype` must be either `Resource` or `Metadata`.
    pub fn new(
        path: &Path,
        resource_type: &str,
        mimetype: &str,
        local_uri: Option<&str>,
        flat_uri: Option<&str>,
        handle: Option<&str>,
    ) -> Self {
        Self::default()
            .with_resource_type(resource_type, mimetype)
            .with_resource_ref(local_uri, flat_uri, handle)
    }

    pub fn with_resource_type(
        &self,
        resource_type: &str,
        mimetype: &str
    ) -> Self {
        Self {
            resource_type: CmdResourceType {
                mimetype: Some(mimetype.to_owned()),
                value: resource_type.to_owned(),
            },
            ..self.to_owned()
        }
    }

    pub fn with_resource_ref(
        &self,
        local_uri: Option<&str>,
        flat_uri: Option<&str>,
        handle: Option<&str>
    ) -> Self {
        Self {
            resource_ref: CmdResourceRef {
                lat_local_uri: local_uri.map(|uri| uri.to_owned()),
                lat_flat_uri: flat_uri.map(|uri| uri.to_owned()),
                value: handle.map(|h| h.to_owned()).unwrap_or_default()
            },
            ..self.to_owned()
        }
    }

    /// FLAT Islandora PID value from attribute `lat:flatURI`.
    /// Possibly contains further info encoded,
    /// such as whether `CMD` or not, and a version time stamp.
    /// e.g. `lat:10050_0f473b55_928f_4c67_9aea_c35f8e9694c8#CMD@2023-01-20T11:34:01.027Z`
    pub fn flat_uri(&self) -> Option<&str> {
        self.resource_ref.lat_flat_uri.as_deref()
    }

    pub fn set_flat_uri(&mut self, uri: &str) {
        self.resource_ref.lat_flat_uri = Some(uri.to_owned())
    }

    /// Returns path to resource specified by `lat:flatURI`.
    /// Note that for an ingested Islandora object this
    /// is never a path, but a PID.
    pub fn flat_uri_as_path(&self, root: Option<&Path>) -> Option<PathBuf> {
        let root = root.unwrap_or(Path::new("."));
        self.flat_uri().map(|uri| root.join(PathBuf::from(uri)))
    }

    /// Returns path to resource specified by `lat:localURI` as `&str`.
    /// Not present for ingested FLAT Islandora objects.
    pub fn local_uri(&self) -> Option<&str> {
        self.resource_ref.lat_local_uri.as_deref()
    }

    pub fn set_local_uri(&mut self, uri: &str) {
        self.resource_ref.lat_local_uri = Some(uri.to_owned())
    }

    /// Returns path to resource specified by `lat:localURI`.
    /// Not present for ingested FLAT Islandora objects.
    pub fn local_uri_as_path(&self, root: Option<&Path>) -> Option<PathBuf> {
        let root = root.unwrap_or(Path::new("."));
        self.local_uri().map(|uri| root.join(PathBuf::from(uri)))
    }
    
    /// FLAT Islandora PID.
    /// Set in optional attribute `lat:flatURI`.
    pub fn pid(&self) -> Option<String> {
        if let Some(uri) = &self.resource_ref.lat_flat_uri {
            // split e.g. `lat:10050_0f473b55_928f_4c67_9aea_c35f8e9694c8#CMD@2023-01-20T11:34:01.027Z`
            uri.split_once('#')
                .map(|spl| spl.0.to_owned())
                .or_else(|| Some(uri.to_owned()))
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

impl Default for ResourceType {
    fn default() -> Self {
        Self::Metadata
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CmdResourceType {
    #[serde(rename="@mimetype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
    // TODO should be enum: Metadata, Resource
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
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
