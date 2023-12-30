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
    let client_db = init_db(client);

    let receiver = RpcHandler::default();
    receiver.receive_message().await;
    // unix domain socket receive stream and send to blocking task spawn to be parsed

    // let test_data = PageInformation {
    //     page_id: "test".to_owned(),
    //     url: "test url".to_owned(),
    //     title: "test title".to_owned(),
    //     description: "test desc".to_owned(),
    //     content_text: "test content".to_owned(),
    // };
    //
    // let result = client_db
    //     .insert_once::<PageInformation>("page_informations", test_data)
    //     .await
    //     .unwrap();
    // println!("{:?}", result);
}


#[derive(Serialize, Deserialize, Debug, Default)]
struct RpcHandler {
    page_html: String,
    tld_id: String,
}

impl RpcHandler {
    async fn receive_message(&self) {
        let listener = UnixListener::bind("/tmp/temp-onecrawl").unwrap();
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let join_handle = handle_client(stream).await;
            let message = join_handle.unwrap();
            println!("{:?}", message);
        }
    }
}

async fn handle_client(mut stream: tokio::net::UnixStream) -> Result<RpcHandler> {
    // Read a message from the client
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await.unwrap();

    // Process the received message (e.g., parse HTML)
    let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received message: {}", received_message);

    // Send a response back to the client
    let response = "Hello from the server!";
    stream.write_all(response.as_bytes()).await.unwrap();

    let deserialized_message: Result<RpcHandler> = serde_json::from_str(&received_message);
    deserialized_message
}
