use actix_web::{web, App, HttpResponse, HttpRequest, Responder, HttpServer};

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/healthcheck", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}