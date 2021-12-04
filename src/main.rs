use sqlx::PgPool;
use std::net::TcpListener;
use whale_watcher_server::configuration::get_configuration;
use whale_watcher_server::startup::run;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Filed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!("Running on 127.0.0.1:{}", configuration.application_port);
    run(listener, connection_pool)?.await
}
