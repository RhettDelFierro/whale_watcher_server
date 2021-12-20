use crate::helpers::spawn_app;
use whale_watcher_server::configuration::Environment::Production;

const ADDRESS: &str = "0x581a92aCE9B720654C03559a9444B5a91e624C00";
const NOTES: &str = "kitty token creator";
const NETWORK_OF_LEGIT_TOKEN: &str = "eth";
const LEGIT_TOKEN_CONTRACT_ADDRESS: &str = "0x044727e50ff30db57fad06ff4f5846eab5ea52a2";

#[derive(serde::Deserialize, Debug)]
struct LegitTokenCreatorResponse {
    data: Vec<LegitTokenCreator>,
}

#[derive(serde::Deserialize, Debug)]
struct LegitTokenCreator {
    address: String,
    notes: String,
    network_of_legit_token: String,
    legit_contract_address: String,
}

#[actix_rt::test]
async fn register_legit_token_creator_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = format!(
        "address={}&notes={}&network_of_legit_token={}&legit_contract_address={}",
        ADDRESS, NOTES, NETWORK_OF_LEGIT_TOKEN, LEGIT_TOKEN_CONTRACT_ADDRESS
    );
    let query_params = format!("token_creator_address={}", ADDRESS,);
    // Act
    let response_post = app.post_legit_token_creators(body.into()).await;

    // Assert
    assert_eq!(200, response_post.status().as_u16());
    let response_get = app.get_legit_token_creators(&query_params).await;
    assert_eq!(200, response_get.status().as_u16());

    let response_parsed = response_get.json::<LegitTokenCreatorResponse>().await;
    let parsed = response_parsed.unwrap();
    assert_eq!(parsed.data[0].address, ADDRESS);
    assert_eq!(parsed.data[0].notes, NOTES);
    assert_eq!(
        parsed.data[0].network_of_legit_token,
        NETWORK_OF_LEGIT_TOKEN
    );
    assert_eq!(
        parsed.data[0].legit_contract_address,
        LEGIT_TOKEN_CONTRACT_ADDRESS
    );
}

#[actix_rt::test]
async fn register_legit_token_creator_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        (
            format!(
                "notes={}&network_of_legit_token={}&legit_contract_address={}",
                NOTES, NETWORK_OF_LEGIT_TOKEN, LEGIT_TOKEN_CONTRACT_ADDRESS
            ),
            "missing the address",
        ),
        (
            format!(
                "address={}&legit_contract_address={}",
                ADDRESS, LEGIT_TOKEN_CONTRACT_ADDRESS
            ),
            "missing network_of_legit_token",
        ),
        (
            format!(
                "address={}&network_of_legit_token={}",
                ADDRESS, NETWORK_OF_LEGIT_TOKEN
            ),
            "missing legit_contract_address",
        ),
        ("".to_string(), "no params"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_legit_token_creators(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[actix_rt::test]
async fn register_legit_token_creator_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            format!(
                "address=&notes={}&network_of_legit_token={}&legit_contract_address={}",
                NOTES, NETWORK_OF_LEGIT_TOKEN, LEGIT_TOKEN_CONTRACT_ADDRESS
            ),
            "empty address",
        ),
        (
            format!(
                "address={}&network_of_legit_token=&legit_contract_address={}",
                ADDRESS, LEGIT_TOKEN_CONTRACT_ADDRESS
            ),
            "empty legit_contract_address",
        ),
        (
            format!(
                "address={}&network_of_legit_token={}&legit_contract_address=",
                ADDRESS, NETWORK_OF_LEGIT_TOKEN
            ),
            "empty legit_contract_address",
        ),
        ("".to_string(), "no params"),
    ];

    for (body, description) in test_cases {
        let response = app.post_legit_token_creators(body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}
