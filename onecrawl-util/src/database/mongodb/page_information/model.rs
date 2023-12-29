use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInformation {
    pub page_id: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub content_text: String,
}
