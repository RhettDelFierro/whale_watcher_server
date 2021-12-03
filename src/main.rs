use std::net::TcpListener;
use whale_watcher_server::configuration::get_configuration;
use whale_watcher_server::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("Preparing to run on {}", address);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
