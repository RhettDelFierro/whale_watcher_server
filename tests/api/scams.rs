use whale_watcher_server::configuration::Environment::Production;
use crate::helpers::spawn_app;

#[derive(serde::Deserialize, Debug)]
struct ScammerResponse {
    data: Vec<Scammer>
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
    let client = reqwest::Client::new();
    let address = "0x18ce832a86C207eeC301437f3dE05Aa11fd79fc1";
    let notes = "ladytigercat creator (honeypot)";
    let network_of_scammed_token = "eth";
    let scammed_contract_address = "0xB91f05B798f8A010A1BDdbFf75dC3D106dC84B50";
    let body = format!(
        "address={}&notes={}&network_of_scammed_token={}&scammed_contract_address={}",
        address,
        notes,
        network_of_scammed_token,
        scammed_contract_address
    );
    let query_params = format!(
        "network={}&scammer_address={}",
        network_of_scammed_token,
        address,
    );
    // Act
    let response_post = app.post_scam_creators(body.into()).await;

    // Assert
    assert_eq!(200, response_post.status().as_u16());
    let response_get = app.get_scam_creators(&query_params).await;
    assert_eq!(200, response_get.status().as_u16());

    let response_get = response_get.json::<ScammerResponse>().await;
    let data = response_get.unwrap();
    assert_eq!(data.data[0].address, address);
    assert_eq!(data.data[0].notes, notes);
    assert_eq!(data.data[0].network_of_scammed_token, network_of_scammed_token);
    assert_eq!(data.data[0].scammed_contract_address, scammed_contract_address);

    // match response_get {
    //     Ok(data) => {
    //         assert_eq!(data.data[0].address, address);
    //         assert_eq!(data.data[0].notes, notes);
    //         assert_eq!(data.data[0].network_of_scammed_token, network_of_scammed_token);
    //         assert_eq!(data.data[0].scammed_contract_address, scammed_contract_address);
    //     },
    //     Err(err) => {
    //
    //     }
    // }
    // let saved = sqlx::query!("SELECT address, notes FROM scam_token_creators",)
    //     .fetch_one(&app.db_pool)
    //     .await
    //     .expect("Failed to fetch saved subscription.");
    //
    // assert_eq!(saved.address, address);
    // assert_eq!(saved.notes, Some(String::from(notes)));
}
//
// #[actix_rt::test]
// async fn register_scammer_returns_a_400_when_data_is_missing() {
//     // Arrange
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();
//     let test_cases = vec![
//         ("name=le%20guin", "missing the email"),
//         ("email=ursula_le_guin%40gmail.com", "missing the name"),
//         ("", "missing both name and email"),
//     ];
//
//     for (invalid_body, error_message) in test_cases {
//         // Act
//         let response = app.post_scam_creators(invalid_body.into()).await;
//
//         // Assert
//         assert_eq!(
//             400,
//             response.status().as_u16(),
//             // Additional customised error message on test failure
//             "The API did not fail with 400 Bad Request when the payload was {}.",
//             error_message
//         );
//     }
// }
//
// #[actix_rt::test]
// async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
//     // Arrange
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();
//     let test_cases = vec![
//         ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
//         ("name=Ursula&email=", "empty email"),
//         ("name=Ursula&email=definitely-not-an-email", "invalid email"),
//     ];
//
//     for (body, description) in test_cases {
//         // Act
//         let response = app.post_scam_creators(body.into()).await;
//
//         // Assert
//         assert_eq!(
//             400,
//             response.status().as_u16(),
//             "The API did not return a 400 Bad Request when the payload was {}.",
//             description
//         );
//     }
// }
