use std::u64;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Crawling {
    pub start_url: String,
    pub keyword: String,
    pub total_page: u64,
    pub duration_crawl: u64,
}
