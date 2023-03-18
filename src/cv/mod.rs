//! Controlled vocabulary, e.g. many booleans often have an "unspecified" value and thus can not be the rust `bool` type.
//! 
//! Quick-xml issue with enums requiring an outer struct "container" to serialize correctly, see:
//! - https://github.com/tafia/quick-xml/issues/430
//! - https://github.com/tafia/quick-xml/issues/346
//! will be/fixed (?) by:
//! - https://github.com/tafia/quick-xml/pull/490 (not yet released in main)

mod boolean;
mod continent;
mod gender;
mod interactivity;
mod planning_type;
mod involvement;
mod social_context;
mod event_structure;
mod channel;

pub use boolean::{Bool, BoolValue};
pub use continent::Continent;
pub use gender::Gender;
pub use interactivity::Interactivity;
pub use planning_type::PlanningType;
pub use involvement::Involvement;
pub use social_context::SocialContext;
pub use event_structure::EventStructure;
pub use channel::{Channel, ChannelValue};