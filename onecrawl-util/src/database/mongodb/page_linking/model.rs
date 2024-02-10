use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageLinking {
    pub page_id: String,
    pub outgoing_link: String,
}
