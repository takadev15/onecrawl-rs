#[path = "http/mod.rs"]
mod http;

use crate::{scouter::thread_manager::http::http_worker::PageScraper, utils::envvars::CrawlerEnv};
use onecrawl_util::database::{
    connection,
    mongodb::{crawling::model::Crawling, init_db},
};
use std::{collections::VecDeque, io::{Error, ErrorKind}, result, sync::{Arc, atomic::{AtomicBool, Ordering}}, thread, time::{Instant, Duration}, u64};
use serde::{Serialize, Deserialize};
use tokio::{runtime::Runtime, net::UnixListener, io::AsyncReadExt};

#[derive(Debug, Default)]
pub struct ThreadManager {
    pub url_list: VecDeque<String>,
    pub url_visited: Vec<String>,
    pub domain_key: String,
}

#[tokio::main]
pub async fn threads_manager(env: &mut CrawlerEnv) {
    let start_time = Instant::now();

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let mut threads_managers: Vec<ThreadManager> = Vec::new();
    let mut threads = Vec::<thread::JoinHandle<()>>::new();

    for (_index, url) in env.url_origin.iter().enumerate() {
        let mut thread_object = ThreadManager::default();
        thread_object.url_list.push_back(url.to_owned());
        thread_object.domain_key = url.to_owned();
        threads_managers.push(thread_object);
    }

    let connection = connection::connect_db("root", "onecrawlrootpass").await;
    let client = init_db(connection);

    let crawl_obj = Crawling{
        start_url: "test".to_owned(),
        keyword: "test".to_owned(),
        total_page: 0,
        duration_crawl: 0,
    };

    let result = client.insert_once("crawling", crawl_obj).await.unwrap();
    let crawl_id = client.get_inserted_id(result.inserted_id).unwrap();

    let mut loop_counter = 0;
    let listener = UnixListener::bind("/tmp/temp-onecrawl-url.sock").unwrap();


    loop {
        let current_time = Instant::now();
        let duration_recorded = current_time - start_time;
        if duration_recorded.as_secs() >= env.crawl_duration {
            println!("crawl stopped at : {}", duration_recorded.as_secs());
            for object in threads_managers {
                println!("last visited urls : {:?}", object.url_visited);
            }
            break;
        }

        if loop_counter > 0 {
            while let Ok((stream, _)) = listener.accept().await {
                let url_rpc_handler = listen_inbound_message(stream).await;
                match url_rpc_handler {
                    Ok(handler ) => {
                        for worker in &mut threads_managers {
                            if worker.domain_key == handler.tld_id {
                                for link in handler.links.to_owned() {
                                    if !worker.url_list.contains(&link) {
                                        worker.url_list.push_front(link);
                                    }
                                }
                            }
                        }
                        break;
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        }
        println!("=== download phases ===");

        let mut thread_counters: u64 = 0;
        for worker in &mut threads_managers {
            if worker.url_list.is_empty() {
                continue;
            }

            let mut page_worker = PageScraper {
                url_list: worker.url_list.to_owned(),
                tld_id: worker.domain_key.to_owned(),
                url_visited: worker.url_visited.to_owned(),
                thread_id: thread_counters + 1,
                crawl_id: crawl_id.to_owned(),
            };

            let url = worker.url_list.pop_front().unwrap();
            worker.url_visited.push(url);
            page_worker.url_visited = worker.url_visited.to_owned();

            let running_clone = running_clone.clone();
            let handle = thread::spawn(move || {
                while running_clone.load(Ordering::Relaxed) {
                    let rt = Runtime::new().unwrap();
                    rt.block_on(page_worker.page_worker()).unwrap();
                }
            });
            threads.push(handle);
            thread_counters = thread_counters + 1;
        }
        // println!("env duration : {}", env.crawl_duration);
        // break;
        loop_counter = loop_counter + 1;
    }

    running.store(false, Ordering::Relaxed);
    println!("crawling duration : {:?}", Instant::now() - start_time);
    for handle in threads {
        handle.join().unwrap_or_default();
    }

}

#[derive(Serialize, Deserialize, Debug)]
struct UrlRpcHandler {
    tld_id: String,
    links: Vec<String>
}

async fn listen_inbound_message(mut stream: tokio::net::UnixStream) -> Result<UrlRpcHandler, Error> {
    // Read a message from the client
    let mut buffer = [0; 1024];
    let mut message = String::new();
    let mut n = 1;

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                return Err(Error::new(ErrorKind::NotFound, "Received empty message"));
            }
            Ok(bytes_read) if bytes_read > 0 => {
                let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
                message.push_str(&chunk);

                // Check for the breakpoint to differetiate messages
                if chunk.contains("/end_link_message") {
                    message.truncate(message.len() - 17);
                    let url_rpc_message: UrlRpcHandler = serde_json::from_str(&message).map_err(|e| {
                        Error::new(ErrorKind::InvalidData, format!("Error passing message: {}", e))
                    })?;

                    message.clear();
                    return Ok(url_rpc_message);
                }
                // println!("loop iteration number {}", n);
                n = n + 1;
            }
            Ok(_) => {
                continue;
            }
            Err(e) => {
                eprintln!("error reading from socket: {}", e);
                return Err(e);
            }
        }
    }
}
