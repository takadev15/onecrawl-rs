use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageImage {
    pub page_id: String,
    pub image: String,
}
