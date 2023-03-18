use serde::{Deserialize, Serialize};

use crate::{Interactivity, PlanningType, Involvement, SocialContext, EventStructure, Channel};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CommunicationContext {
    #[serde(rename = "Interactivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    interactivity: Option<Interactivity>,
    #[serde(rename = "PlanningType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    planning_type: Option<PlanningType>,
    #[serde(rename = "Involvement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    involvement: Option<Involvement>,
    #[serde(rename = "SocialContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    social_context: Option<SocialContext>,
    #[serde(rename = "EventStructure", default)]
    event_structure: Vec<EventStructure>,
    #[serde(rename = "Channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<Channel>,
}
