use crate::util::env;
use std::{
    fs,
    time::{Duration, Instant},
};
// use std::os::unix::net::{UnixListener, UnixStream};
use onecrawl_util::database::{
    connection,
    mongodb::{init_db, MongoDB},
};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use tokio::net::UnixListener;
use tokio::time::timeout;

#[path = "worker/worker.rs"]
mod worker;

pub async fn parser_controller() {
    let mut env = env::ParserEnv::default();
    env.load_env();

    let client = connection::connect_db("root", "onecrawlrootpass").await;
    let client_db = init_db(client);

    let start_time = Instant::now();
    let timeout_duration = Duration::from_secs(env.parsing_duration + 30);
    let listener = UnixListener::bind("/tmp/temp-onecrawl-page.sock").unwrap();

    listen_message(listener, &client_db, start_time, timeout_duration).await;

    if let Err(err) = fs::remove_file("/tmp/temp-onecrawl-page.sock") {
        eprintln!("Failed to remove page UDS file: {}", err);
    }

    if let Err(err) = fs::remove_file("/tmp/temp-onecrawl-url.sock") {
        eprintln!("Failed to remove url UDS file: {}", err);
    }
}

async fn listen_message(
    listener: UnixListener,
    client: &MongoDB,
    start_time: Instant,
    timeout_duration: Duration,
) {
    while start_time.elapsed() < timeout_duration {
        match timeout(timeout_duration - start_time.elapsed(), listener.accept()).await {
            Ok(Ok((mut stream, _))) => {
                println!("Accepted connection");

                let mut message = String::new();
                loop {
                    let mut buf = vec![0; 1024];
                    let bytes_read = stream.read(&mut buf).await.unwrap();
                    if bytes_read == 0 {
                        break; // End of stream
                    }
                    let chunk = String::from_utf8_lossy(&buf[..bytes_read]);
                    if chunk.contains("/end_crawled_message") {
                        message.push_str(&chunk);
                        break;
                    }
                    message.push_str(&chunk);
                }

                // Process the message (Deserialized the message, parse the HTML)
                process_message(&mut message, client).await;
            }
            Ok(Err(e)) => {
                eprintln!("Error accepting connection: {:?}", e);
            }
            Err(_) => {
                println!("Timeout reached. Exiting.");
                break;
            }
        }
    }
}

async fn process_message(message: &mut String, client: &MongoDB) {
    message.truncate(message.len() - 20);
    let mut rpc_message: RpcHandler = serde_json::from_str(message).unwrap();
    rpc_message.parse_html(client).await;
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RpcHandler {
    page_html: String,
    tld_id: String,
    visited_url: Vec<String>,
    crawl_id: String,
}
