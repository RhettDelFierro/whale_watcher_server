use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct WhaleData {
    token_name: String,
    contract_address: String,
    holder_address: String,
    place: u16,
    amount: f64
}

async fn add_whale(_form: web::Form<WhaleData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/add_whale", web::post().to(add_whale))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
