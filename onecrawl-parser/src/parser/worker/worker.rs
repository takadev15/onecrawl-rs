use onecrawl_util::database::mongodb::{
    page_form::model::PageForm, page_images::model::PageImage, page_information,
    page_linking::model::PageLinking, page_list::model::PageList, page_scripts::model::PageScript,
    page_styles::model::PageStyle, page_tables::model::PageTables, MongoDB,
};
use regex::Regex;
use select::document::Document;
use select::predicate::{Name, Text};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::str::FromStr;
use url::Url;

use super::RpcHandler;

impl RpcHandler {
    pub async fn parse_html(&mut self, client: &MongoDB) {
        println!("Parsed url : {}", self.visited_url.last().unwrap());
        let document = Document::from(self.page_html.as_str());

        let mut html5 = false;
        let mut text = String::new();
        if document.find(Name("article")).count() > 0 {
            if let Some(article) = document.find(Name("article")).next() {
                let mut visible_tex: Vec<&str> = Vec::new();
                for node in article.find(Text) {
                    if Regex::new(r"[\n]+")
                        .unwrap()
                        .is_match(node.as_text().unwrap())
                    {
                        continue;
                    } else if Regex::new(r"[\t]+")
                        .unwrap()
                        .is_match(node.as_text().unwrap())
                    {
                        continue;
                    } else {
                        visible_tex.push(node.as_text().unwrap());
                    }
                }
                text = visible_tex.join(",")
            }
            html5 = true;
        } else {
            if let Some(body) = document.find(Name("body")).next() {
                let mut visible_tex: Vec<&str> = Vec::new();
                for node in body.find(Text) {
                    if tag_visible(node) {
                        visible_tex.push(node.as_text().unwrap());
                    }
                }
                text = visible_tex.join(",")
            }
        }

        let mut description = "-";
        if let Some(meta_description) = document
            .find(Name("meta"))
            .find(|n| n.attr("name") == Some("description"))
        {
            description = meta_description.attr("content").unwrap();
            println!("{}", description);
        }

        let mut keyword = "-";
        if let Some(meta_keyword) = document
            .find(Name("meta"))
            .find(|n| n.attr("name") == Some("keyword"))
        {
            keyword = meta_keyword.attr("content").unwrap();
            println!("{}", keyword);
        }

        let mut title = String::from_str("-").unwrap();
        if let Some(head_title) = document.find(Name("title")).next() {
            title = head_title.text();
        }

        let title = title.to_owned();
        let page_info_object = page_information::model::PageInformation {
            url: self.visited_url.last().unwrap().to_string(),
            html5,
            title,
            description: description.to_owned(),
            keywords: keyword.to_owned(),
            content_text: text,
            size_bytes: 200,
        };
        let result = client
            .insert_once("page_informations", page_info_object)
            .await
            .unwrap();
        let page_id = client.get_inserted_id(result.inserted_id).unwrap();
        println!("{}", page_id);

        let mut links: Vec<String> = Vec::new();
        let mut page_link_objects: Vec<PageLinking> = Vec::new();
        for link in document.find(Name("a")) {
            if let Some(href) = link.attr("href") {
                if href.starts_with("http://") || href.starts_with("https://") {
                    let base_url = Url::parse(&self.tld_id).unwrap();
                    let outgoing_link = Url::parse(href).unwrap();

                    let domain_check = outgoing_link
                        .host_str()
                        .map_or(false, |host| host.ends_with(base_url.host_str().unwrap()));
                    if domain_check {
                        if !self.visited_url.contains(&href.to_string()) {
                            let page_link = PageLinking {
                                page_id: page_id.to_owned(),
                                outgoing_link: href.to_string(),
                            };
                            links.push(href.to_string());
                            page_link_objects.push(page_link);
                        }
                    }
                }
            }
        }
        let rpc_message = UrlRpcHandler {
            tld_id: self.tld_id.to_owned(),
            links,
        };
        rpc_message.send_message();

        if !page_link_objects.is_empty() {
            client.insert_bulk("page_linking", page_link_objects).await;
        }
        let mut page_list_objects: Vec<PageList> = Vec::new();
        for list in document.find(Name("li")) {
            let page_list = PageList {
                page_id: page_id.to_owned(),
                list: list.html(),
            };
            page_list_objects.push(page_list);
        }
        if !page_list_objects.is_empty() {
            client.insert_bulk("page_lists", page_list_objects).await;
        }

        let mut page_table_objects: Vec<PageTables> = Vec::new();
        for table in document.find(Name("table")) {
            let page_table = PageTables {
                page_id: page_id.to_owned(),
                table_str: table.html(),
            };
            page_table_objects.push(page_table);
        }
        if !page_table_objects.is_empty() {
            client.insert_bulk("page_tables", page_table_objects).await;
        }

        let mut page_form_objects: Vec<PageForm> = Vec::new();
        for form in document.find(Name("form")) {
            let page_form = PageForm {
                page_id: page_id.to_owned(),
                form: form.html(),
            };
            page_form_objects.push(page_form);
        }
        if !page_form_objects.is_empty() {
            client.insert_bulk("page_forms", page_form_objects).await;
        }

        let mut page_image_objects: Vec<PageImage> = Vec::new();
        for image in document.find(Name("img")) {
            let page_image = PageImage {
                page_id: page_id.to_owned(),
                image: image.html(),
            };
            page_image_objects.push(page_image);
        }
        if !page_image_objects.is_empty() {
            client.insert_bulk("page_images", page_image_objects).await;
        }

        let mut page_style_objects: Vec<PageStyle> = Vec::new();
        for style in document.find(Name("style")) {
            let page_style = PageStyle {
                page_id: page_id.to_owned(),
                style: style.html(),
            };
            page_style_objects.push(page_style);
        }
        if !page_style_objects.is_empty() {
            client.insert_bulk("page_styles", page_style_objects).await;
        }

        let mut page_script_objects: Vec<PageScript> = Vec::new();
        for script in document.find(Name("script")) {
            let page_script = PageScript {
                page_id: page_id.to_owned(),
                script: script.html(),
            };
            page_script_objects.push(page_script);
        }
        if !page_script_objects.is_empty() {
            client
                .insert_bulk("page_scripts", page_script_objects)
                .await;
        }
    }
}

pub fn tag_visible(element: select::node::Node) -> bool {
    let parent_names = ["style", "script", "head", "title", "meta", "[document]"];
    if let Some(parent_name) = element.parent().and_then(|p| p.name()) {
        if parent_names.contains(&parent_name) {
            return false;
        }
    }
    if Regex::new(r"[\n]+")
        .unwrap()
        .is_match(element.as_text().unwrap())
    {
        return false;
    }

    if Regex::new(r"[\t]+")
        .unwrap()
        .is_match(element.as_text().unwrap())
    {
        return false;
    }

    true
}

#[derive(Serialize, Deserialize, Debug)]
struct UrlRpcHandler {
    tld_id: String,
    links: Vec<String>,
}

impl UrlRpcHandler {
    fn send_message(&self) {
        let serialized_message = serde_json::to_string(&self).unwrap() + "/end_link_message";
        // println!("{}", serialized_message);
        let mut stream = UnixStream::connect("/tmp/temp-onecrawl-url.sock").unwrap();
        stream.write_all(serialized_message.as_bytes()).unwrap();
    }
}
