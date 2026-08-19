use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::dev::Server;
use actix_web::web::Form;
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData{
    name:String,
    email:String
}

pub async fn healthcheck() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn subscribe(_form:Form<FormData>) -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub fn run(listener:TcpListener) -> Result<Server, std::io::Error>{
    let server = HttpServer::new(||{
        App::new().route("/healthcheck", web::get().to(healthcheck))
        .route("/subscribe",web::post().to(subscribe))
    }).listen(listener)?.run();
    Ok(server)
}