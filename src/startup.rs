use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
use crate::routes::{add_holder,health_check};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/holders", web::post().to(add_holder))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
