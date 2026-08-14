use rust_newsletter_api::run;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works(){
    let address=spawn_app();
    let client = reqwest::Client::new();
    let response = client.get(format!("{}/healthcheck",address)).send().await.expect("Request failed");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(),Some(0));
}

fn spawn_app()->String{
    let listener=TcpListener::bind("127.0.0.1:0").expect("Binding failed");
    let port=listener.local_addr().unwrap().port();
    let server=rust_newsletter_api::run(listener).expect("Binding failed");
    let _=tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)
}