use crate::helpers::spawn_app;
use serde_json::{from_str, Value};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HolderRowData {
    network: String,
    contract_address: String,
    holder_address: String,
    notes: Option<String>,
    address_type: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HolderDescriptionsResponse {
    pub data: Vec<HolderRowData>,
}
#[actix_rt::test]
async fn add_holder_descriptions_returns_a_200_for_validform_data() {
    let app = spawn_app().await;
    let body = r#"{
        "network": "bsc",
        "holder_descriptions": [
            {"holder_address": "someholderaddress1", "contract_address": 'somecontractaddress1, "notes": "holder1 notes", "address_type": ["whale", "longterm_holder"]},
            {"holder_address": "someholderaddress2", "contract_address": 'somecontractaddress1, "notes": "holder2 notes", "address_type": ["longterm_holder", "token_creator"]},
            {"holder_address": "someholderaddress3", "contract_address": 'somecontractaddress1, "notes": "holder3 notes", "address_type": ["scammer", "paperhand", "dumper"]},
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
    assert_eq!(parsed.data[0].network, "bsc");
    assert_eq!(parsed.data[0].holder_address, "someholderaddress1");
}
//
// #[actix_rt::test]
// async fn add_holder_descriptions_returns_a_400_when_data_is_missing() {
//     let app = spawn_app().await;
//     let no_contract_address = r#"{
//         "network": "bsc",
//         "token_name": "some coin",
//         "holders": [{"holder_address": "someholderaddress", "place": 10, "amount": 10.10}]
//     }"#;
//     let no_token_name = r#"{
//         "network": "bsc",
//         "contract_address": "some contract address",
//         "holders": [{"holder_address": "someholderaddress", "place": 10, "amount": 10.10}]
//     }"#;
//     let no_network = r#"{
//         "token_name": "some coin",
//         "contract_address": "some contract address",
//         "holders": [{"holder_address": "someholderaddress", "place": 10, "amount": 10.10}]
//     }"#;
//     let no_holders = r#"{
//         "network": "bsc",
//         "token_name": "some coin",
//         "contract_address": "some contract address"
//     }"#;
//     let test_cases = vec![
//         (no_contract_address, "no contract address"),
//         (no_token_name, "no token name"),
//         (no_network, "no network"),
//         (no_holders, "no holders"),
//     ];
//
//     for (invalid_body, error_message) in test_cases {
//         let v: Value = serde_json::from_str(invalid_body).unwrap();
//         let response = app.post_holders(&v).await;
//
//         assert_eq!(
//             400,
//             response.status().as_u16(),
//             "The API did not fail with 400 Bad Request when the payload was {}.",
//             error_message
//         );
//     }
// }
//
// #[actix_rt::test]
// async fn insert_holder_description_fails_if_there_is_a_fatal_database_error() {
//     let app = spawn_app().await;
//     let body = r#"{
//         "network": "bsc",
//         "token_name": "some coin",
//         "contract_address": "some contract address",
//         "holders": [{"holder_address": "some holder address", "place": 10, "amount": 10.10}]
//     }"#;
//     let v: Value = serde_json::from_str(body).unwrap();
//     sqlx::query!("ALTER TABLE holder_totals DROP COLUMN holder_address",)
//         .execute(&app.db_pool)
//         .await
//         .unwrap();
//     let response_post = app.post_holders(&v).await;
//     assert_eq!(response_post.status().as_u16(), 500);
// }
