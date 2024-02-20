use std::{env, u64};
use dotenv::dotenv;

#[derive(Debug, Default)]
pub struct ParserEnv {
    pub parsing_duration: u64,
    pub thread_count: u64,
}

impl ParserEnv {
    pub fn load_env(&mut self) {
        dotenv().ok();

        match env::var("PARSER_MAX_THREADS") {
            Ok(val) => self.thread_count = val.parse::<u64>().unwrap(),
            Err(e) => println!("couldn't interpret CRAWLER_START_URLS : {e}"),
        }

        match env::var("PARSER_DURATION_SECONDS") {
            Ok(val) => self.parsing_duration = val.parse::<u64>().unwrap(),
            Err(e) => println!("couldn't interpret CRAWLER_START_URLS : {e}"),
        }
    }
}
