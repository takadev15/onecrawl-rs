#[path = "http/mod.rs"]
mod http;

use crate::{scouter::thread_manager::http::http_worker::PageScraper, utils::envvars::CrawlerEnv};
use std::{collections::VecDeque, thread, time::{Instant, Duration}, u64, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use tokio::runtime::Runtime;

#[derive(Debug, Default)]
pub struct ThreadManager {
    pub url_list: VecDeque<String>,
    pub url_visited: Vec<String>,
    pub domain_key: u64,
}

pub fn threads_manager(env: &mut CrawlerEnv) {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let mut threads_managers: Vec<ThreadManager> = Vec::new();
    let mut threads = Vec::<thread::JoinHandle<()>>::new();

    let start_time = Instant::now();

    for (index, url) in env.url_origin.iter().enumerate() {
        let mut thread_object = ThreadManager::default();
        thread_object.url_list.push_back(url.to_owned());
        thread_object.domain_key = index as u64 + 1;

        threads_managers.push(thread_object);
    }

    loop {
        let current_time = Instant::now();
        let duration_recorded = current_time - start_time;
        if duration_recorded.as_secs() >= env.crawl_duration {
            println!("crawl stopped at : {}", duration_recorded.as_secs());
            break;
        }

        for worker in &threads_managers {
            if worker.url_list.is_empty() {
                continue;
            }
            let mut page_worker = PageScraper {
                url_list: worker.url_list.to_owned(),
                // tld_id: worker.url_list.pop_front(),
                url_visited: worker.url_visited.to_owned(),
                thread_id: worker.domain_key,
            };

            let running_clone = running_clone.clone();
            let handle = thread::spawn(move || {
                while running_clone.load(Ordering::Relaxed) {
                    let rt = Runtime::new().unwrap();
                    let result = rt.block_on(page_worker.page_worker());
                }
            });
            threads.push(handle);
        }
        println!("env duration : {}", env.crawl_duration);
        // break;
    }

    running.store(false, Ordering::Relaxed);
    println!("crawling duration : {:?}", Instant::now() - start_time);
    for handle in threads {
        handle.join().unwrap();
    }

}

fn listen_message() {

}
