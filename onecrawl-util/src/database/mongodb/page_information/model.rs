use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInformation {
    // pub crawl_id: u32,
    pub url: String,
    pub html5: bool,
    pub title: String,
    pub description: String,
    pub keywords: String,
    pub content_text: String,
    pub size_bytes: u32,
}
