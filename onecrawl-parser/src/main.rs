mod parser;

use onecrawl_util::{add, database};

fn main() {
    println!("Hello, world!");
    let num = add(10, 10);
    let client = database::connection::connect_db();
    let connected = database::connection::ping_db(client, "crawler");

    // let page_coll: Collection<Pages> = client.database("crawler").collection("pages");
    if connected {
        println!("connected");
    } else if !connected {
        println!("not connected");
    }
    println!("{}", num);
}
