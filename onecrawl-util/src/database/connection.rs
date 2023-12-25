use mongodb::{options::ClientOptions, bson::{doc, Document}, sync::Client};

pub fn connect_db() -> Client {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    let client = Client::with_options(client_options).unwrap();

    client
}

pub fn ping_db(client: Client, db_name: &str) -> bool {
    let connection = client.database(db_name).run_command(doc! {"ping": 1}, None);
    match connection {
        Ok(_) => true,
        Err(err_message) => {
            println!("ping rejected: {}", err_message);
            false
        },
    }
}
