use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use whale_watcher_server::configuration::get_configuration;

// our integration test
// basically going to run this test like it was a real user:
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let server = whale_watcher_server::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn holder_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failted to read configuration.");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    let body = "token_name=kitty&contract_address=0x044727e50ff30db57fad06ff4f5846eab5ea52a2&holder_address=0x53084957562b692ea99beec870c12e7b8fb2d28e&place=2&amount=27939322392%2E330572392&timestamp=2000";

    let response = client
        .post(&format!("{}/holders", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT holder_address FROM holders",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(
        saved.holder_address,
        "0x53084957562b692ea99beec870c12e7b8fb2d28e"
    )
}

#[actix_rt::test]
async fn holder_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("token_name=kitty", "token_name"),
        (
            "contract_address=0x044727e50ff30db57fad06ff4f5846eab5ea52a2",
            "missing contract_address",
        ),
        (
            "holder_address=0x53084957562b692ea99beec870c12e7b8fb2d28e",
            "missing holder address",
        ),
        ("place=2", "missing place"),
        ("amount=27939322392%2E330572392", "missing amount"),
        ("timestamp=2000", "missing amount"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/holders", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
