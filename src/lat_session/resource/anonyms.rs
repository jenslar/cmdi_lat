use serde::{Deserialize, Serialize};

use super::access::Access;

// TODO add fields
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Anonyms{
    #[serde(rename="Access")]
    access: Access
}