//! Core structure representing CMDI metadata used by MPI Nijmegen's FLAT archiving software.
//!
//! FLAT Bundle, a.k.a. `lat-session`.
//! - CMDI v1.1 profile [`clarin.eu:cr1:p_1407745712035`](https://catalog.clarin.eu/ds/ComponentRegistry#/?itemId=clarin.eu%3Acr1%3Ap_1407745712035&registrySpace=public)
//! - [CMDI v1.1 schema](https://catalog.clarin.eu/ds/ComponentRegistry/rest/registry/1.1/profiles/clarin.eu:cr1:p_1407745712035/xsd)

pub mod location;
pub mod project;
pub mod content;
pub mod actor;
pub mod resource;
pub mod lat_session;

pub use lat_session::LatSession;
pub use project::Project;
pub use location::{GeoCoordinates, Location};
pub use content::{
    ContentLanguage,
    ContentLanguages,
    CommunicationContext,
};
pub use actor::{
    Actor,
    Actors,
    ActorLanguage,
    ActorLanguages,
    Age,
    AgeRange,
    ExactAge
};