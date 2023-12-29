use std::u64;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Crawling {
    id_crawling: String,
    start_url: String,
    keyword: String,
    total_page: u64,
    duration_crawl: String,
}
