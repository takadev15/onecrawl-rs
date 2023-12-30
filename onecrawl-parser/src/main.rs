mod parser;

use parser::parser_controller;


#[tokio::main]
async fn main() {
    parser_controller().await
}
