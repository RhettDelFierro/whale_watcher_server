use whale_watcher_server::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    println!("Running on http://127.0.0.1:{}/", port);
    run(listener)?.await
}
