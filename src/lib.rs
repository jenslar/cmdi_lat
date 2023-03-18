//! De/serialization for CMDI profiles XXX and YYY, which are based on the earlier IMDI standard. Specifically for MPI Nijmegens FLAT archiving system, developed by The Language Archive (URL).
//! For an overview of core unit types CMDI/lat-session uses see: https://clarin.oeaw.ac.at/smc-browser/data/smc_stats_datcat1.html

mod cmd;
mod lat_corpus;
mod lat_session;
mod key;
mod contact;
mod description;
mod resources;
mod cv;
mod lingmetax;
mod tree;
mod info_link;
mod defaults;
mod errors;

pub use cmd::Cmd;
pub use cmd::Header;
pub use cmd::{Components, ComponentType};
pub use lat_corpus::LatCorpus;
pub use lat_session::LatSession;
pub use lat_session::{
    Actor,
    Actors,
    ActorLanguage,
    ActorLanguages,
};
pub use lat_session::content::{
    Content,
    ContentLanguage,
    ContentLanguages,
};
pub use lat_session::project::Project;
pub use lat_session::location::{Location, GeoCoordinates};
pub use description::{Description, Descriptions};
pub use tree::Tree;
pub use defaults::Defaults;
pub use cv::{
    Bool,
    BoolValue,
    Interactivity,
    Continent,
    Gender,
    PlanningType,
    Involvement,
    SocialContext,
    EventStructure,
    Channel,
    ChannelValue
};
pub use resources::{
    CmdResources,
    CmdResourceProxyList,
    CmdResourceProxy,
    CmdResourceRef,
    CmdResourceType,
    CmdJournalFileProxyList,
    CmdJournalFileProxy,
    CmdResourceRelationList,
    CmdResourceRelation
};