use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn not_found() -> impl Responder {
    HttpResponse::Ok().body("Not Found!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(hello))
            .default_service(
                web::route().to(not_found)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}