use crate::helpers::spawn_app;

#[actix_rt::test]
async fn holders_returns_a_200_for_validform_data() {
    let app = spawn_app().await;
    let body = "network=ethereum&token_name=kitty&contract_address=0x044727e50ff30db57fad06ff4f5846eab5ea52a2&holder_address=0x53084957562b692ea99beec870c12e7b8fb2d28e&place=2&amount=27939322392%2E330572392";

    let response = app.post_holders(body.into()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT holder_address FROM holder_totals",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(
        saved.holder_address,
        "0x53084957562b692ea99beec870c12e7b8fb2d28e"
    );
}

#[actix_rt::test]
async fn holders_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
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
        let response = app.post_holders(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
