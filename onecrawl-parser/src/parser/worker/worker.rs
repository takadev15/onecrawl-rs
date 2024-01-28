use select::document::{self, Document};
use select::predicate::{Attr, Name, Predicate};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::{borrow::Borrow, io::Read};
use tokio::io::AsyncWriteExt;
use url::Url;

use super::RpcHandler;

impl RpcHandler {
    pub fn parse_html(&mut self) {
        let document = Document::from(self.page_html.as_str());
        let mut links: Vec<String> = Vec::new();
        for link in document.find(Name("a")) {
            if let Some(href) = link.attr("href") {
                if href.starts_with("http://") || href.starts_with("https://") {
                    let base_url = Url::parse(&self.tld_id).unwrap();
                    let outgoing_link = Url::parse(href).unwrap();

                    let domain_check = outgoing_link
                        .host_str()
                        .map_or(false, |host| host.ends_with(base_url.host_str().unwrap()));
                    if domain_check {
                        links.push(href.to_string());
                    }
                }
            }
        }

        for list in document.find(Name("li")) {
            println!("LIST");
            println!("{}", list.html());
        }

        for table in document.find(Name("table")) {
            println!("TABLE");
            println!("{}", table.html());
        }

        let rpc_message = UrlRpcHandler {
            tld_id: self.tld_id.to_owned(),
            links: links,
        };
        rpc_message.send_message();
        // println!("{:?}", links);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct UrlRpcHandler {
    tld_id: String,
    links: Vec<String>,
}

impl UrlRpcHandler {
    fn send_message(&self) {
        let serialized_message = serde_json::to_string(&self).unwrap() + "/end_link_message";
        println!("{}", serialized_message);
        let mut stream = UnixStream::connect("/tmp/temp-onecrawl-url.sock").unwrap();
        stream.write_all(serialized_message.as_bytes()).unwrap();
    }
}
