use rust_newsletter_api::run;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener=TcpListener::bind("127.0.0.1:8000").expect("Failed to bind");
    run(listener)?.await
}