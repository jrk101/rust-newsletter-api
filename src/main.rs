use rust_newsletter_api::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}