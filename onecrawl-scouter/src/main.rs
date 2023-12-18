use std::str::FromStr;

pub mod scouter;
pub mod utils;

#[tokio::main]
async fn main() {
    // load .env
    let mut env = utils::envvars::CrawlerEnv::default();
    env.load_env();

    // run scouter's service
    scouter::thread_manager::threads_manager(&mut env).await;
}
