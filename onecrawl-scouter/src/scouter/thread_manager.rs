#[path = "http/mod.rs"]
mod http;
use std::{collections::VecDeque, i32, u64};

use self::http::http_worker;
use crate::{scouter::thread_manager::http::http_worker::PageScraper, utils::envvars::CrawlerEnv};
use tokio::task;

#[derive(Debug, Default)]
pub struct ThreadManager {
    pub url_list: VecDeque<String>,
    pub url_visited: Vec<String>,
    pub domain_key: u64,
}

pub async fn threads_manager(env: &mut CrawlerEnv) {
    let mut threads_managers: Vec<ThreadManager> = Vec::new();

    for (index, url) in env.url_origin.iter().enumerate() {
        let mut thread_object = ThreadManager::default();
        thread_object.url_list.push_back(url.to_owned());
        thread_object.domain_key = index as u64 + 1;

        threads_managers.push(thread_object);
    }

    for worker in threads_managers {
        let mut page_worker = PageScraper {
            url_list: worker.url_list,
            url_visited: worker.url_visited,
            thread_id: worker.domain_key,
        };

        let join = task::spawn(page_worker.page_worker()).await;
    }
}
