use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::TcpListener;
use whale_watcher_server::configuration::get_configuration;
use whale_watcher_server::email_client::EmailClient;
use whale_watcher_server::startup::run;
use whale_watcher_server::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "whale_watcher_server".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    let sender_email_address = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email_address,
        configuration.email_client.authorization_token,
    );

    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
