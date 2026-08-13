use rust_newsletter_api::run;

#[tokio::test]
async fn health_check_works(){
    spawn_app();
    let client = reqwest::Client::new();
    let response = client.get("http://127.0.0.1:8000/healthcheck").send().await.expect("Request failed");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(),Some(0));
}

fn spawn_app(){
    let server=rust_newsletter_api::run().expect("Binding failed");
    let _=tokio::spawn(server);
}