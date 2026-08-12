use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::dev::Server;

pub async fn healthcheck() -> impl Responder{
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error>{
    let server = HttpServer::new(||{
        App::new().route("/healthcheck", web::get().to(healthcheck))
    }).bind("127.0.0.1:8000")?.run();
    Ok(server)
}