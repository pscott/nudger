use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nudge {
    pub protocol: String,
    pub cta: String,

    #[serde(skip_serializing)]
    pub filter_name: String,
}
