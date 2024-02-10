use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageList {
    pub page_id: String,
    pub list: String,
}
