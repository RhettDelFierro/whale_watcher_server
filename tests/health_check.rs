#[actix_rt::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// our integration test
// basically going to run this test like it was a real user:
fn spawn_app() {
    let server = whale_watcher_server::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
