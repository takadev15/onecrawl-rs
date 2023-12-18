use std::{env, u64};

use dotenv::dotenv;

#[derive(Debug, Default)]
pub struct CrawlerEnv {
    pub url_origin: Vec<String>,
    pub url_origin_count: u64,
    pub crawl_duration: u64,
    pub thread_count: u64,
}

impl CrawlerEnv {
    pub fn load_env(&mut self) {
        dotenv().ok();

        match env::var("CRAWLER_MAX_THREADS") {
            Ok(val) => self.thread_count = val.parse::<u64>().unwrap(),
            Err(e) => println!("couldn't interpret CRAWLER_START_URLS : {e}"),
        }

        match env::var("CRAWLER_DURATION_SECONDS") {
            Ok(val) => self.crawl_duration = val.parse::<u64>().unwrap(),
            Err(e) => println!("couldn't interpret CRAWLER_START_URLS : {e}"),
        }

        match env::var("CRAWLER_START_URLS") {
            Ok(val) => {
                for url in val.split_whitespace() {
                    self.url_origin.push(url.to_string());
                }
            }
            Err(e) => println!("couldn't interpret CRAWLER_START_URLS : {e}"),
        }

        self.url_origin_count = self.url_origin.len() as u64;
    }
}
