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

#[tokio::test]
async fn returns_200_ok_for_valid_data(){
    let address=spawn_app();
    let client=reqwest::Client::new();
    let body="name=joseph%20rithin&email=josephrithin2004%40gmail.com";
    let response=client.post(&format!("{}/subscribe",address)).header("Content-Type","application/x-www-form-urlencoded").body(body).send().await.expect("Faied to send data");
    assert_eq!(200,response.status().as_u16());
}

#[tokio::test]
async fn returns_400_for_missing_data(){
    let address=spawn_app();
    let client=reqwest::Client::new();
    let test_results=vec![("name=joseph%20rithin","email is missing"),("mail=josephrithin2004%40gmail.com","Name is missing"),("","Both data is missing")];

    for (invalid_body,error_msg) in test_results{
        let response=client.post(&format!("{}/subscribe",address)).header("Content-Type","application/x-www-form-urlencoded").body(invalid_body).send().await.expect("failed");
        assert_eq!(400,response.status().as_u16(),"{}",error_msg);
    }
}