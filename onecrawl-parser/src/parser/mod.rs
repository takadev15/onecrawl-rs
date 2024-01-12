use std::sync::{Arc, Mutex};

use html5ever::tendril::stream;
use onecrawl_util::database::{
    connection,
    mongodb::{init_db, page_information::model::PageInformation, MongoDB},
};
use serde::{Serialize, Deserialize};
use serde_json::Result;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[path = "worker/worker.rs"]
mod worker;

pub async fn parser_controller() {
    // unimplemented!();
    let client = connection::connect_db("root", "onecrawlrootpass").await;
    // let client_db = init_db(client);

    let receiver = RpcHandler::default();

    let listener = UnixListener::bind("/tmp/temp-onecrawl.sock").unwrap();
    while let Ok((stream, _)) = listener.accept().await {
        handle_client(stream).await;
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RpcHandler {
    page_html: String,
    tld_id: String,
}

impl RpcHandler {
}

async fn handle_client(mut stream: tokio::net::UnixStream) {
    // Read a message from the client
    let mut buffer = [0; 1024];
    let mut message = String::new();

    loop {
        match stream.read(&mut buffer).await {
            Ok(bytes_read) if bytes_read > 0 => {
                let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
                message.push_str(&chunk);

                // Check for the breakpoint to differetiate messages
                if chunk.contains("/end_crawled_message") {
                    // Pass the message to a blocking thread for processing
                    // let processing_result = tokio::task::spawn_blocking(move || {
                    //     // Process the message in a blocking thread
                    //     process_message(&message)
                    // }).await.unwrap();
                    process_message(&message);

                    // Clear the message buffer for the next message
                    message.clear();
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("error reading from socket: {}", e);
                break;
            }
        }
    }
}

fn process_message(message: &str) {
    println!("{}", message);
}
