use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nudge {
    pub protocol: String,
    pub cta_url: String,
    pub cta_text: String,

    #[serde(skip_serializing)]
    pub filter_name: String,
}
