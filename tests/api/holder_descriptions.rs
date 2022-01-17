use crate::helpers::spawn_app;
use serde_json::{from_str, Value};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HolderRowData {
    network_name: String,
    contract_address: String,
    holder_address: String,
    notes: String,
    address_types: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HolderDescriptionsResponse {
    pub data: Vec<HolderRowData>,
}
#[actix_rt::test]
async fn add_holder_descriptions_returns_a_200_for_validform_data() {
    let app = spawn_app().await;
    let body = r#"{
        "network_name": "bsc",
        "holder_descriptions": [
            {"holder_address": "someholderaddress1", "contract_address": "somecontractaddress1", "notes": "holder1 notes", "address_types": ["whale", "longterm_holder"]},
            {"holder_address": "someholderaddress2", "contract_address": "somecontractaddress1", "notes": "holder2 notes", "address_types": ["longterm_holder", "token_creator"]},
            {"holder_address": "someholderaddress3", "contract_address": "somecontractaddress1", "notes": "holder3 notes", "address_types": ["scammer", "paperhand", "dumper"]}
        ]
    }"#;
    let v: Value = serde_json::from_str(body).unwrap();
    let response = app.post_holder_descriptions(&v).await;

    assert_eq!(200, response.status().as_u16());

    let fetch_body = r#"{
        "holder_addresses": ["someholderaddress1"]
    }"#;
    let fetch_v: Value = serde_json::from_str(fetch_body).unwrap();
    let response_get = app.get_holder_descriptions(&fetch_v).await;
    assert_eq!(200, response_get.status().as_u16());

    let response_parsed = response_get.json::<HolderDescriptionsResponse>().await;
    let parsed = response_parsed.unwrap();
    assert_eq!(parsed.data[0].contract_address, "somecontractaddress1");
    assert_eq!(parsed.data[0].notes, "holder1 notes");
    assert_eq!(parsed.data[0].network_name, "bsc");
    assert_eq!(parsed.data[0].holder_address, "someholderaddress1");
    assert_eq!(parsed.data[0].address_types[0], "whale");
}

#[actix_rt::test]
async fn add_holder_descriptions_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let no_network_name = r#"{
        "holder_descriptions": [
            {"holder_address": "someholderaddress1", "contract_address": "somecontractaddress1", "notes": "holder1 notes", "address_types": ["whale", "longterm_holder"]},
            {"holder_address": "someholderaddress2", "contract_address": "somecontractaddress1", "notes": "holder2 notes", "address_types": ["longterm_holder", "token_creator"]},
            {"holder_address": "someholderaddress3", "contract_address": "somecontractaddress1", "notes": "holder3 notes", "address_types": ["scammer", "paperhand", "dumper"]}
        ]
    }"#;
    let no_holder_address = r#"{
        "network_name": "bsc",
        "holder_descriptions": [
            {"contract_address": "somecontractaddress1", "notes": "holder1 notes", "address_types": ["whale", "longterm_holder"]}
        ]
    }"#;
    let no_contract_address = r#"{
        "network_name": "bsc",
        "holder_descriptions": [
            {"holder_address": "someholderaddress3", "notes": "holder3 notes", "address_types": ["scammer", "paperhand", "dumper"]}
        ]
    }"#;
    let no_address_types = r#"{
        "network_name": "bsc",
        "holder_descriptions": [
            {"holder_address": "someholderaddress3", "contract_address": "somecontractaddress1", "notes": "holder3 notes"}
        ]
    }"#;
    let test_cases = vec![
        (no_network_name, "no network name"),
        (no_contract_address, "no contract address"),
        (no_holder_address, "no holder address"),
        (no_address_types, "no address types"),
    ];

    for (invalid_body, error_message) in test_cases {
        let v: Value = serde_json::from_str(invalid_body).unwrap();
        let response = app.post_holder_descriptions(&v).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[actix_rt::test]
async fn insert_holder_description_fails_if_there_is_a_fatal_database_error() {
    let app = spawn_app().await;
    let body = r#"{
        "network_name": "bsc",
        "holder_descriptions": [
            {"holder_address": "someholderaddress1", "contract_address": "somecontractaddress1", "notes": "holder1 notes", "address_types": ["whale", "longterm_holder"]},
            {"holder_address": "someholderaddress2", "contract_address": "somecontractaddress1", "notes": "holder2 notes", "address_types": ["longterm_holder", "token_creator"]},
            {"holder_address": "someholderaddress3", "contract_address": "somecontractaddress1", "notes": "holder3 notes", "address_types": ["scammer", "paperhand", "dumper"]}
        ]
    }"#;
    let v: Value = serde_json::from_str(body).unwrap();
    sqlx::query!("ALTER TABLE holder_descriptions DROP COLUMN contract_address",)
        .execute(&app.db_pool)
        .await
        .unwrap();
    let response_post = app.post_holder_descriptions(&v).await;
    assert_eq!(response_post.status().as_u16(), 500);
}
