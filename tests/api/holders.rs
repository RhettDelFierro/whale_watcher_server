use crate::helpers::spawn_app;
use bigdecimal::ToPrimitive;
use serde_json::{from_str, Value};
use sqlx::types::BigDecimal;

#[actix_rt::test]
async fn holders_returns_a_200_for_validform_data() {
    let app = spawn_app().await;
    let body = r#"{
        "network": "bsc",
        "token_name": "some coin",
        "contract_address": "some contract address",
        "holders": [{"holder_address": "someholderaddress", "place": 10, "amount": "10,000,000,000,000.100001"}]
    }"#;
    let v: Value = serde_json::from_str(body).unwrap();
    let response = app.post_holders(&v).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT holder_address FROM holder_totals",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.holder_address, "someholderaddress");

    let saved_amount = sqlx::query!("SELECT amount FROM holder_totals",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved_amount.amount.to_f64().unwrap(), 10000000000000.100001);
}

#[actix_rt::test]
async fn holders_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let no_contract_address = r#"{
        "network": "bsc",
        "token_name": "some coin",
        "holders": [{"holder_address": "someholderaddress", "place": 10, "amount": "10.10"}]
    }"#;
    let no_token_name = r#"{
        "network": "bsc",
        "contract_address": "some contract address",
        "holders": [{"holder_address": "someholderaddress", "place": 10, "amount": "10.10"}]
    }"#;
    let no_network = r#"{
        "token_name": "some coin",
        "contract_address": "some contract address",
        "holders": [{"holder_address": "someholderaddress", "place": 10, "amount": "10.10"}]
    }"#;
    let no_holders = r#"{
        "network": "bsc",
        "token_name": "some coin",
        "contract_address": "some contract address"
    }"#;
    let test_cases = vec![
        (no_contract_address, "no contract address"),
        (no_token_name, "no token name"),
        (no_network, "no network"),
        (no_holders, "no holders"),
    ];

    for (invalid_body, error_message) in test_cases {
        let v: Value = serde_json::from_str(invalid_body).unwrap();
        let response = app.post_holders(&v).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[actix_rt::test]
async fn add_holder_fails_if_there_is_a_fatal_database_error() {
    let app = spawn_app().await;
    let body = r#"{
        "network": "bsc",
        "token_name": "some coin",
        "contract_address": "some contract address",
        "holders": [{"holder_address": "some holder address", "place": 10, "amount": "10.10"}]
    }"#;
    let v: Value = serde_json::from_str(body).unwrap();
    sqlx::query!("ALTER TABLE holder_totals DROP COLUMN holder_address",)
        .execute(&app.db_pool)
        .await
        .unwrap();
    let response_post = app.post_holders(&v).await;
    assert_eq!(response_post.status().as_u16(), 500);
}
