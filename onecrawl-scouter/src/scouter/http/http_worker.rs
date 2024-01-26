use dotenv::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, time::Duration};
use tokio::{io::AsyncWriteExt, net::{UnixListener, UnixStream}, time::Instant};

#[derive(Serialize, Deserialize, Debug)]
struct RpcMessage {
    page_html: String,
    tld_id: String,
}

#[derive(Debug, Default)]
pub struct PageScraper {
    pub url_list: VecDeque<String>,
    pub tld_id: String,
    pub url_visited: Vec<String>,
    pub thread_id: u64,
}

impl PageScraper {
    pub async fn page_worker(&mut self) -> Result<()> {
        println!("thread id : {}", self.thread_id);
        match self.url_list.pop_front().as_deref() {
            Some(url) => {
                let start = Instant::now();
                let client = Client::new();
                let resp: String = client
                    .get(url)
                    .send()
                    .await
                    .expect("failed to get response")
                    .text()
                    .await
                    .expect("failed to get payload");
                let duration = start.elapsed();
                // println!("{}", resp);
                println!("download duration: {:?}", duration);

                let send_message = RpcMessage {
                    page_html: resp,
                    tld_id: self.tld_id.to_owned(),
                } ;
                send_message.send_message().await;
            }
            None => {
                println!("no url left");
            }
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
        Ok(())
    }
}

impl RpcMessage {
    async fn send_message(&self) {
        let serialized_message = serde_json::to_string(&self).unwrap() + "/end_crawled_message";
        println!("{}", serialized_message);
        // let listener = UnixListener::bind("/tmp/temp-onecrawl-url.sock").unwrap();
        // let addr = listener.local_addr().unwrap();
        let mut stream = UnixStream::connect("/tmp/temp-onecrawl-page.sock").await.unwrap();
        stream
            .write_all(serialized_message.as_bytes())
            .await
            .unwrap();
        drop(stream);
    }
}
