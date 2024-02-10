use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageScript {
    pub page_id: String,
    pub script: String,
}
