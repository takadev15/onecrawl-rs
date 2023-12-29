use mongodb::{options::ClientOptions, bson::{doc, Document}, Client};

pub async fn connect_db(uname: &str, pass: &str) -> Client {
    let url = format!("mongodb://{}:{}@localhost:27017/", uname, pass);
    let client_options = ClientOptions::parse(url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    client
}

pub async fn ping_db(client: &Client, db_name: &str) -> bool {
    let connection = client.database(db_name).run_command(doc! {"ping": 1}, None).await;
    match connection {
        Ok(_) => true,
        Err(err_message) => {
            println!("ping rejected: {}", err_message);
            false
        },
    }
}
