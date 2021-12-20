use crate::helpers::spawn_app;
use whale_watcher_server::configuration::Environment::Production;

const ADDRESS: &str = "0x18ce832a86C207eeC301437f3dE05Aa11fd79fc1";
const NOTES: &str = "ladytigercat creator (honeypot)";
const NETWORK_OF_SCAMMED_TOKEN: &str = "eth";
const SCAMMED_TOKEN_ADDRESS: &str = "0xB91f05B798f8A010A1BDdbFf75dC3D106dC84B50";

#[derive(serde::Deserialize, Debug)]
struct ScammerResponse {
    data: Vec<Scammer>,
}

#[derive(serde::Deserialize, Debug)]
struct Scammer {
    address: String,
    notes: String,
    network_of_scammed_token: String,
    scammed_contract_address: String,
}

#[actix_rt::test]
async fn register_scammer_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = format!(
        "address={}&notes={}&network_of_scammed_token={}&scammed_contract_address={}",
        ADDRESS, NOTES, NETWORK_OF_SCAMMED_TOKEN, SCAMMED_TOKEN_ADDRESS
    );
    let query_params = format!("token_creator_address={}", ADDRESS,);
    // Act
    let response_post = app.post_scam_creators(body.into()).await;

    // Assert
    assert_eq!(200, response_post.status().as_u16());
    let response_get = app.get_scam_creators(&query_params).await;
    assert_eq!(200, response_get.status().as_u16());

    let response_parsed = response_get.json::<ScammerResponse>().await;
    let parsed = response_parsed.unwrap();
    assert_eq!(parsed.data[0].address, ADDRESS);
    assert_eq!(parsed.data[0].notes, NOTES);
    assert_eq!(
        parsed.data[0].network_of_scammed_token,
        NETWORK_OF_SCAMMED_TOKEN
    );
    assert_eq!(
        parsed.data[0].scammed_contract_address,
        SCAMMED_TOKEN_ADDRESS
    );
}

#[actix_rt::test]
async fn register_scammer_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        (
            format!(
                "notes={}&network_of_scammed_token={}&scammed_contract_address={}",
                NOTES, NETWORK_OF_SCAMMED_TOKEN, SCAMMED_TOKEN_ADDRESS
            ),
            "missing the address",
        ),
        (
            format!(
                "address={}&scammed_contract_address={}",
                ADDRESS, SCAMMED_TOKEN_ADDRESS
            ),
            "missing network_of_scammed_token",
        ),
        (
            format!(
                "address={}&network_of_scammed_token={}",
                ADDRESS, NETWORK_OF_SCAMMED_TOKEN
            ),
            "missing scammed_contract_address",
        ),
        ("".to_string(), "no params"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_scam_creators(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[actix_rt::test]
async fn register_scammer_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        (
            format!(
                "address=&notes={}&network_of_scammed_token={}&scammed_contract_address={}",
                NOTES, NETWORK_OF_SCAMMED_TOKEN, SCAMMED_TOKEN_ADDRESS
            ),
            "empty address",
        ),
        (
            format!(
                "address={}&network_of_scammed_token=&scammed_contract_address={}",
                ADDRESS, SCAMMED_TOKEN_ADDRESS
            ),
            "empty scammed_contract_address",
        ),
        (
            format!(
                "address={}&network_of_scammed_token={}&scammed_contract_address=",
                ADDRESS, NETWORK_OF_SCAMMED_TOKEN
            ),
            "empty scammed_contract_address",
        ),
        ("".to_string(), "no params"),
    ];

    for (body, description) in test_cases {
        let response = app.post_scam_creators(body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}
