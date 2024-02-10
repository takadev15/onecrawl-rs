use std::{io::Read, sync::{mpsc, Arc, Mutex}, thread};

use onecrawl_util::database::{
    connection,
    mongodb::{init_db, page_information::model::PageInformation, MongoDB},
};
use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::os::unix::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[path = "worker/worker.rs"]
mod worker;

pub async fn parser_controller() {
    // unimplemented!();
    let client = connection::connect_db("root", "onecrawlrootpass").await;
    let client_db = init_db(client);

    let receiver = RpcHandler::default();

    let listener = UnixListener::bind("/tmp/temp-onecrawl-page.sock").unwrap();
    while let Ok((stream, _)) = listener.accept() {
        handle_client(stream, &client_db).await;
    }
}

async fn handle_client(mut stream: std::os::unix::net::UnixStream, client: &MongoDB) {
    // Read a message from the client
    let mut buffer = [0; 1024];
    let mut message = String::new();
    let mut n = 1;

    // let (sender, receiver) = mpsc::channel();
    // let receiver = Arc::new(Mutex::new(receiver));
    //
    // let mut handler = Vec::<thread::JoinHandle<()>>::new();
    // for _ in 0..2 {
    //     let receiver = Arc::clone(&receiver);
    //     let handle = thread::spawn(move || {
    //         receiver.lock().unwrap().recv();
    //     });
    //     handler.push(handle);
    // }

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                break;
            }
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
                    // sender.send(process_message(&mut message, client));
                    // thread::scope(|scope| {
                    //     scope.spawn(|| process_message(&mut message, client));
                    // });
                    process_message(&mut message, client).await;

                    // Clear the message buffer for the next message
                    message.clear();
                }
                n = n + 1;
            }
            Ok(_) => {
                continue;
            }
            Err(e) => {
                eprintln!("error reading from socket: {}", e);
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
}
