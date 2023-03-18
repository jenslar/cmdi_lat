use std::path::{Path, PathBuf, Iter};

use serde::{Deserialize, Serialize};

use crate::{Header, ComponentType, Description, Actor};

use crate::errors::CmdError;
use crate::resources::{CmdResources, CmdResourceProxy};
use crate::Defaults;

use super::components::Components;

/// Struct for CMDI-profiles
/// [clarin.eu:cr1:p_1407745712064](http://catalog.clarin.eu/ds/ComponentRegistry/rest/registry/profiles/clarin.eu:cr1:p_1407745712064)
/// (`lat-corpus`) and
/// [clarin.eu:cr1:p_1407745712035](http://catalog.clarin.eu/ds/ComponentRegistry/rest/registry/profiles/clarin.eu:cr1:p_1407745712035) (`lat-session`)
/// 
/// Used in MPI Nijmegen's [FLAT Archiving software](https://github.com/TLA-FLAT/FLAT),
/// developed by [The Language Archive](https://archive.mpi.nl/tla/).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "CMD")]
pub struct Cmd {
    /// Path to input file
    #[serde(skip)]
    pub path: PathBuf,
    
    // Root level attributes, de/serialize via defaults instead, since
    // quick-xml can't (de?)serialize root level namespace,
    // see: https://github.com/tafia/quick-xml/pull/539

    #[serde(rename = "@xmlns:xsi", default="Defaults::xmlns_xsi")]
    xmlns_xsi: String,
    #[serde(rename = "@xmlns", default="Defaults::xmlns")]
    xmlns: String,
    #[serde(rename = "@xmlns:cmd", default="Defaults::xmlns_cmd")]
    xmlns_cmd: String,
    #[serde(rename = "@xmlns:imdi", default="Defaults::xmlns_imdi")]
    xmlns_imdi: String,
    #[serde(rename = "@xmlns:lat", default="Defaults::xmlns_lat")]
    xmlns_lat: String,
    #[serde(rename = "@xmlns:iso", default="Defaults::xmlns_iso")]
    xmlns_iso: String,
    #[serde(rename = "@xmlns:sil", default="Defaults::xmlns_sil")]
    xmlns_sil: String,
    #[serde(rename = "@xmlns:xs", default="Defaults::xmlns_xs")]
    xmlns_xs: String,
    #[serde(rename = "@xmlns:functx", default="Defaults::xmlns_functx")]
    xmlns_functx: String,
    #[serde(rename = "@CMDVersion")]
    cmd_version: String,
    #[serde(rename = "@xsi:schemaLocation", default="Defaults::xsi_schemalocation")]
    xsi_schema_location: String,

    // Content

    /// CMDI header.
    #[serde(rename="Header")]
    pub header: Header,
    
    /// Linked resources.
    #[serde(rename="Resources")]
    pub resources: CmdResources,
    
    /// Contains metadata, mostly following the IMDI standard.
    /// Can be a session (`lat-session`, links data)
    /// or a corpus (`lat-corpus`, links sessions).
    #[serde(rename="Components")]
    pub components: Components,
}

// struct CmdIter<'a> {
//     // Absolute path to CMDI file
//     path: PathBuf,
//     // resouces: Vec<CmdResourceProxy>
//     items: Box<dyn Iterator<Item = &'a CmdResourceProxy>>
// }

// impl Iterator for CmdIter {
//     type Item = Cmd;

//     fn next(&mut self) -> Option<Self::Item> {
//         let cmd = Cmd::deserialize(&self.path).ok()?;
//         cmd.resources.resource_proxy_list.resource_proxies.iter().next()
//     }
// }

// impl<'a> Iterator for CmdIter<'a> {
//     type Item = &'a Cmd;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(cmd_iter) = self.iter.next() {
//             match cmd_iter.cmd.resource_type() {
//                 "Metadata" => {
//                     let rel_path = Path::new(res.local_uri()?);
//                     let abs_path = self.path.parent()?.join(rel_path);
//                     Cmd::deserialize(&abs_path).ok()
//                 },
//                 _ => None
//             }
//         }
//     }
// }

struct CmdIter {
    base_path: PathBuf,
    /// Derived from base_path (i.e. root)
    paths: Vec<PathBuf>,
    children: Vec<Self>
}

// impl CmdIter {
//     pub fn new(cmd: &Cmd) -> Self {
//         Self {
//             base_path: cmd.path.to_owned(), path: (), children: () }
//     }
// }

impl Iterator for Cmd {
    type Item = Cmd;

    fn next(&mut self) -> Option<Self::Item> {
        self.resources.resource_proxy_list.resource_proxies.iter().next()
            .and_then(|proxy| {
                dbg!(proxy);
                match proxy.resource_type() {
                    "Metadata" => {
                        let rel_path = Path::new(proxy.local_uri()?);
                        let abs_path = self.path.parent()?.join(rel_path);
                        Cmd::deserialize(&abs_path).ok()
                    },
                    _ => None
                }
            })
            
        // if let Some(cmd) = self.iter_cmd_local().next() {
        // if let Some(proxy) = self.resources.resource_proxy_list.resource_proxies.iter().next() {
        //     match proxy.resource_type() {
        //         "Metadata" => {
        //             let rel_path = Path::new(proxy.local_uri()?);
        //             let abs_path = self.path.parent()?.join(rel_path);
        //             Cmd::deserialize(&abs_path).ok()
        //         },
        //         _ => None
        //     }
        //     // cmd.resources.resource_proxy_list.resource_proxies.iter().next()
        // } else {
        //     None
        // }
        // if let Some(res) = self.resources().next() {
        //     match res.resource_type() {
        //         "Metadata" => {
        //             let rel_path = Path::new(res.local_uri()?);
        //             let abs_path = self.path.parent()?.join(rel_path);
        //             Cmd::deserialize(&abs_path).ok()
        //         },
        //         _ => None
        //     }
        // } else {
        //     None
        // }
    }
}

struct CmdPaths {
    path: PathBuf,
    children: Vec<PathBuf>
}

struct CmdTree {
    paths: CmdPaths,
    children: Vec<Self>
}

impl Default for Cmd {
    fn default() -> Self {
        Self {
            path:                PathBuf::default(),
            xmlns_xsi:           Defaults::xmlns_xsi(),
            xmlns:               Defaults::xmlns(),
            xmlns_cmd:           Defaults::xmlns_cmd(),
            xmlns_imdi:          Defaults::xmlns_imdi(),
            xmlns_lat:           Defaults::xmlns_lat(),
            xmlns_iso:           Defaults::xmlns_iso(),
            xmlns_sil:           Defaults::xmlns_sil(),
            xmlns_xs:            Defaults::xmlns_xs(),
            xmlns_functx:        Defaults::xmlns_functx(),
            cmd_version:         "1.1".to_owned(),
            xsi_schema_location: Defaults::xsi_schemalocation(),
            header: Header::default(),
            resources: CmdResources::default(),
            components: Components::default(),
            // components: vec![Components::default()],
            // components: ComponentType::default(),
        }
    }
}

impl Cmd {
    pub fn iter_chain(&self) {
        for cmd in self.children() {
            println!("{:?} {}", cmd.header.md_self_link, cmd.path.canonicalize().unwrap().display());
            for res in cmd.resources() {
                println!("  {:?}", res);
                // if res.resource_type() == "Resource" {
                //     println!("  {}", res.resource_ref.value)
                // }
            }
            cmd.iter_chain()
        }
    }

    /// Returns an iterator over linked `Metadata` child nodes,
    /// returning each child node path as `Cmd` if valid CMDI.
    /// Requires `lat:localURI` to be set to a valid path.
    /// 
    /// Note that ingested Islandora object do not have `lat:localURI` set.
    /// In this case use `Cmd::iter_cmd_flat()` instead (requires
    /// a mapping between handle and local path).
    pub fn children<'a>(&'a self) -> impl Iterator<Item = Cmd> + 'a {
        self.resources()
            .filter_map(|res| if res.is_metadata() {
                res.local_uri_path(self.path.parent())
                    .and_then(|p| Cmd::deserialize(&p).ok())
            } else {
                None
            })
            .into_iter()
    }

    /// Returns paths to all linked metadata/CMD
    /// (`resource_type = Metadata`).
    pub fn paths_metadata(&self) -> Vec<PathBuf> {
        self.resources()
            .filter_map(|res| if res.is_metadata() {
                res.local_uri_path(self.path.parent())
            } else {
                None
            })
            .collect()
    }

    /// Returns paths to all linked data
    /// (`resource_type = Resource`).
    pub fn paths_resource(&self) -> Vec<PathBuf> {
        self.resources()
            .filter_map(|res| if res.is_resource() {
                res.local_uri_path(self.path.parent())
            } else {
                None
            })
            .collect()
    }

    // pub fn children_paths<'a>(&'a self) -> impl Iterator<Item = PathBuf> + 'a {
    // pub fn children_paths(&self) -> impl Iterator<Item = PathBuf> {
    //     self.paths_metadata().into_iter()
    //         .chain(
    //             &mut self.paths_metadata()
    //                 .iter()
    //                 .filter_map(|p| Cmd::deserialize(&p)
    //                     .map(|cmd| cmd.children_paths()).ok()
    //                 )
    //                 .into_iter()
    //         )
    // }

    /// Iterate over CMD tree, using a pre-compiled
    /// SQLite database tha maps handle to local path.
    fn iter_cmd_flat(&self, sqlite_db: &Path) {
        for res in self.resources() {
            if let Some(uri) = res.local_uri() {
                if let Ok(cmd) = Self::deserialize(&Path::new(uri)) {
                    // cmd.iter_flat_uri()
                }
            }
        }
    }

    // fn iter_local_uri(&self) {
    //     for res in self.resources() {
    //         // res.
    //     }
    // }

    pub fn from_slice(slice: &[u8]) -> Result<Self, CmdError> {
        quick_xml::de::from_str::<Cmd>(&String::from_utf8_lossy(slice))
            .map_err(|err| CmdError::DeError(err))
    }
    /// Deserialize CMD-file with profiles
    /// 
    pub fn deserialize(path: &Path) -> Result<Self, CmdError> {
        let mut cmd = quick_xml::de::from_str::<Cmd>(&std::fs::read_to_string(path)?)
            .map_err(|err| CmdError::DeError(err))?;
        cmd.path = path.to_owned();
        Ok(cmd)
    }

    /// Serialize CMD struct to single-line string.
    // pub fn serialize(&self) -> Result<String, quick_xml::DeError> {
    pub fn serialize(&self) -> Result<String, CmdError> {
        // Serializing to string outputs result as a single line, currently no indent options in quick-xml
        // so use e.g. xmllint --format if indentation is needed.
        quick_xml::se::to_string(&self)
            // TODO undo need for ugly string replace below (due to enum serialization bug in quick-xml).
            // TODO 230207 no longer needed as of quick-xml 0.27.1?
            .map(|s| s.replace("<MediaFile><MediaFile>", "<MediaFile>"))
            .map(|s| s.replace("</MediaFile></MediaFile>", "</MediaFile>"))
            .map(|s| s.replace("<Source><Source>", "<Source>"))
            .map(|s| s.replace("</Source></Source>", "</Source>"))
            .map(|s| s.replace("<lat-session><lat-session>", "<lat-session>"))
            .map(|s| s.replace("</lat-session></lat-session>", "</lat-session>"))
            .map(|s| s.replace("<lat-corpus><lat-corpus>", "<lat-corpus>"))
            .map(|s| s.replace("</lat-corpus></lat-corpus>", "</lat-corpus>"))
            .map_err(|err| CmdError::DeError(err))
    }

    /// Returns a iterator over `ResourceProxy` elements.
    /// A `ResourceProxy` holds links to resources,
    /// which can point to either more CMDI-files (`resource_type` = `Metadata`),
    /// or data files (`resource_type` = `Resource`).
    pub fn resources(&self) -> impl Iterator<Item = &CmdResourceProxy> {
        self.resources.resource_proxy_list.resource_proxies.iter()
    }

    pub fn add_resource(&mut self) {

    }

    /// Remove all `Keys` fields present.
    /// I.e. all `Keys` present will be set to `None`.
    pub fn delete_keys(&mut self) {
        match &mut self.components.component_type {
            ComponentType::LatCorpus(_corpus) => (), // no Keys found
            ComponentType::LatSession(session) => {
                session.keys = None;
                session.content.keys = None;
                if let Some(content_lang) = &mut session.content.content_languages {
                    content_lang.languages.iter_mut()
                        .for_each(|l| l.keys = None)
                }
                session.actors.actors.iter_mut()
                    .for_each(|a| a.keys = None)
            },
        }
    }

    pub fn date(&self) -> Option<String> {
        match &self.components.component_type {
            ComponentType::LatSession(ses) => Some(ses.date.to_owned()),
            ComponentType::LatCorpus(_) => None,
        }
    }

    /// Get top-level descriptions
    pub fn description(&self) -> Vec<Description> {
        match &self.components.component_type {
            ComponentType::LatSession(ses) => ses.descriptions.as_ref().map(|d| d.values()).into_iter().flatten().collect::<Vec<_>>(),
            ComponentType::LatCorpus(crp) => crp.descriptions.as_ref().map(|d| d.values()).into_iter().flatten().collect::<Vec<_>>(),
        }
    }

    /// Get all actors if `lat-session`.
    /// `lat-corpus` returns empty vector.
    pub fn actors(&self) -> Vec<Actor> {
        match &self.components.component_type {
            ComponentType::LatSession(ses) => ses.actors.actors.to_owned(),
            ComponentType::LatCorpus(_) => Vec::new(),
        }
    }
}