use std::{collections::VecDeque};

use reqwest::Client;
use tokio::time::Instant;

#[derive(Debug, Default)]
pub struct PageScraper {
    pub url_list: VecDeque<String>,
    pub url_visited: Vec<String>,
    pub thread_id: u64,
}

impl PageScraper {
    pub async fn page_worker(mut self) {
        println!("thread id : {}", self.thread_id);
        match self.url_list.pop_front().as_deref() {
            Some(url) => {
                let start = Instant::now();
                let client = Client::new();
                let resp = client
                    .get(url)
                    .send()
                    .await
                    .expect("failed to get response")
                    .text()
                    .await
                    .expect("failed to get payload");
                let duration = start.elapsed();
                println!("{}", resp);
                println!("time: {:?}", duration);
            },
            None => {
                println!("no url left");
            }
        }
    }

    fn send_body(mut self) {
        unimplemented!();
    }

    fn download_page(mut self) {
        unimplemented!();
    }
}

mod page_downloader {

}
