use html5ever::parse_document;
use html5ever::tendril::fmt::Slice;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use serde::{Serialize, Deserialize};
use tokio::io::AsyncWriteExt;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::default::Default;
use std::{borrow::Borrow, io::Read};
use url::Url;

use super::RpcHandler;

impl RpcHandler {
    pub fn parse_html(&mut self) {
        let parser = parse_document(RcDom::default(), Default::default());
        let dom = parser
            .from_utf8()
            .read_from(&mut self.page_html.as_bytes())
            .unwrap();
        let mut links: Vec<String> = Vec::new();
        self.walk(0, &dom.document, &mut links);

        if !dom.errors.is_empty() {
            println!("\nParse errors:");
            for err in dom.errors.iter() {
                println!("{}", err);
            }
        }
        let rpc_message = UrlRpcHandler{
            tld_id: self.tld_id.to_owned(),
            links: links,
        };
        rpc_message.send_message();
        // println!("{:?}", links);
    }

    fn walk(&mut self, indent: usize, handle: &Handle, links_array: &mut Vec<String>) {
        let node = handle;
        for _ in 0..indent {
            print!(" ");
        }
        match node.data {
            NodeData::Document => println!("#Document"),

            NodeData::Doctype {
                ref name,
                ref public_id,
                ref system_id,
            } => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

            NodeData::Text { ref contents } => {
                println!("#text: {}", contents.borrow().escape_default())
            }

            NodeData::Comment { ref contents } => {
                println!("<!-- {} -->", contents.escape_default())
            }

            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                // print!("<{}", name.local);
                if &*name.local == "a" {
                    if let Some(href) = attrs
                        .borrow()
                        .iter()
                        .find(|attr| &*attr.name.local == "href")
                    {
                        if &*href.value != "#" {
                            let base_url = Url::parse(&self.tld_id).unwrap();
                            let outgoing_link = Url::parse(&*href.value).unwrap();
                            let domain_check = outgoing_link
                                .host_str()
                                .map_or(false, |host| host.ends_with(base_url.host_str().unwrap()));
                            if domain_check {
                                links_array.push(href.value.to_string());
                            }
                        }
                    }
                } else if &*name.local == "table"{
                        println!("{:?}", attrs);
                    }
                // for attr in attrs.borrow().iter() {
                //     print!(" {}=\"{}\"", attr.name.local, attr.value);
                // }
                // println!(">");
            }

            NodeData::ProcessingInstruction { .. } => unreachable!(),
        }

        for child in node.children.borrow().iter() {
            self.walk(indent + 4, child, links_array);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct UrlRpcHandler {
    tld_id: String,
    links: Vec<String>
}

impl UrlRpcHandler {
    fn send_message(&self) {
        let serialized_message = serde_json::to_string(&self).unwrap() + "/end_link_message";
        println!("{}", serialized_message);
        let mut stream = UnixStream::connect("/tmp/temp-onecrawl-url.sock").unwrap();
        stream.write_all(serialized_message.as_bytes()).unwrap();
    }
}
