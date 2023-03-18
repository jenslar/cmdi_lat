use serde::{Deserialize, Serialize};

use super::{mediafile::MediaFile, writtenresource::WrittenResource, source::Source, anonyms::Anonyms};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LatResources {
    // raises error on '<Resources/>' (missing field `$value`), using Option<Vec<Resource>> parses fewer files
    // seems related to de/ser of enums
    // possibly fixed in https://github.com/tafia/quick-xml/pull/490 (not yet released)
    #[serde(rename = "$value", default)]
    // #[serde(rename = "Resource")]
    resources: Vec<LatResource>,
    // returns 'duplicate field `$value`' as Option<> Element required by xsd, but no content ok, need quick-xml to read/write empty element
    // resources: Option<Vec<Resource>>
}

/// LAT resource, linked files.
#[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(rename="$untagged")] seems not to make a difference?
pub enum LatResource {
    // #[serde(rename = "MediaFile")]
    MediaFile(MediaFile),
    // #[serde(rename = "WrittenResource")]
    WrittenResource(WrittenResource),
    // #[serde(rename = "Source")]
    Source(Source),
    Anonyms(Anonyms),
}

impl Default for LatResource {
    fn default() -> Self {
        Self::MediaFile(MediaFile::default())
    }
}
