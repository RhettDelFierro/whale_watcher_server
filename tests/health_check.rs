use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use whale_watcher_server::configuration::{get_configuration, DatabaseSettings};
use whale_watcher_server::startup::run;
use whale_watcher_server::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test".into(), "debug".into());
    init_subscriber(subscriber);
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// our integration test
// basically going to run this test like it was a real user:
async fn spawn_app() -> TestApp {
    Lazy:: force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

#[actix_rt::test]
async fn health_check_works() {
    let test_app: TestApp = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn holder_returns_a_200_for_validform_data() {
    let test_app: TestApp = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "network=ethereum&token_name=kitty&contract_address=0x044727e50ff30db57fad06ff4f5846eab5ea52a2&holder_address=0x53084957562b692ea99beec870c12e7b8fb2d28e&place=2&amount=27939322392%2E330572392&timestamp=2000";

    let response = client
        .post(&format!("{}/holders", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT address FROM addresses",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.address, "0x044727e50ff30db57fad06ff4f5846eab5ea52a2")
}

#[actix_rt::test]
async fn holder_returns_a_400_when_data_is_missing() {
    let test_app: TestApp = spawn_app().await;
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
        ("", "missing all required field"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/holders", test_app.address))
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
