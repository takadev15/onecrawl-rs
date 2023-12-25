use dotenv::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, time::Duration};
use tokio::{net::UnixStream, io::AsyncWriteExt, time::Instant};

#[derive(Serialize, Deserialize, Debug)]
struct RpcMessage {
    page_html: String,
    tld_id: String,
}

#[derive(Debug, Default)]
pub struct PageScraper {
    pub url_list: VecDeque<String>,
    // pub tld_id: String,
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
            }
            None => {
                println!("no url left");
            }
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
        Ok(())
    }

    async fn send_message(self, body: &str, tld: &str) {
        let message = RpcMessage {
            page_html: body.to_string(),
            tld_id: tld.to_string(),
        };
        let serialized_message = serde_json::to_string(&message).unwrap();
        let mut stream = UnixStream::connect("/tmp/temp-onecrawl").await.unwrap();
        stream
            .write_all(serialized_message.as_bytes())
            .await
            .unwrap();
    }
}
