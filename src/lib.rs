use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

pub async fn healthcheck() -> impl Responder{
    HttpResponse::Ok()
}

pub fn run(listener:TcpListener) -> Result<Server, std::io::Error>{
    let server = HttpServer::new(||{
        App::new().route("/healthcheck", web::get().to(healthcheck))
    }).listen(listener)?.run();
    Ok(server)
}