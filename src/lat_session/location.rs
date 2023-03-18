use serde::{Deserialize, Serialize};

use crate::Continent;

/// Location.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Location {
    #[serde(rename = "Continent")]
    // pub continent: Continent,
    pub continent: String, // TODO use Enum Continent below
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Region", default)]
    pub region: Vec<String>,
    // #[serde(skip_serializing_if = "Option::is_none")] // or serialize anyawy?
    // pub region: Option<Vec<String>>,
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")] // or serialize anyawy?
    pub address: Option<String>,
    #[serde(rename = "geoCoordinates", default)]
    // #[serde(default)]
    pub coordinates: Vec<GeoCoordinates>
    // #[serde(skip_serializing_if = "Option::is_none")] // or serialize anyawy?
    // pub coordinates: Option<Vec<GeoCoordinates>>
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub enum Continent {
//     Africa,
//     Asia,
//     Europe,
//     Australia,
//     Oceania,
//     #[serde(rename="North-America")]
//     NorthAmerica,
//     #[serde(rename="Middle-America")]
//     MiddleAmerica,
//     #[serde(rename="South-America")]
//     SouthAmerica,
//     Unspecified,
//     Unknown
// }

// impl Default for Continent {
//     fn default() -> Self {
//         Self::Unspecified
//     }
// }

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename="camelCase")]
pub struct GeoCoordinates {
    #[serde(rename="Latitude")]
    pub latitude: f64,
    #[serde(rename="Longitude")]
    pub longitude: f64,
    #[serde(rename="Elevation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
}