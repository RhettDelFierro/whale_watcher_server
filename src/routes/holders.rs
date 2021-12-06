use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct HolderData {
    network: String,
    token_name: String,
    // TODO: will still need to make a separate table
    contract_address: String,
    // TODO: will still need to make a separate table
    holder_address: String,
    place: u16,
    // TODO: will still need to make a separate table
    amount: f64,
    // TODO: will still need to make a separate table
    timestamp: i64, // TODO: will be from Utc::now().timestamp() from a time passed in by the front end.
}

pub async fn add_holder(form: web::Form<HolderData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_span = tracing::info_span!(
        "Adding a new holder.",
        network = %form.network,
        contract_address= %form.contract_address
    );
    let _request_span_guard = request_span.enter();

    let network_query_span = tracing::info_span!("Saving new network details in the database");
    tracing::info!("Adding {} to networks.", form.network);
    match sqlx::query!(
        r#"
        INSERT INTO networks (network_name) VALUES ($1) ON CONFLICT DO NOTHING;
        "#,
        form.network
    )
    .execute(pool.get_ref())
    .instrument(network_query_span)
    .await
    {
        Ok(_) => {
            let address_query_span =
                tracing::info_span!("Saving new address details in the database");
            match sqlx::query!(
                r#"
                INSERT INTO addresses (network_id, address)
                VALUES (
                 (SELECT network_id FROM networks WHERE network_name = $1),
                 $2
                );
                "#,
                form.network,
                form.contract_address
            )
            .execute(pool.get_ref())
            .instrument(address_query_span)
            .await
            {
                Ok(_) => {
                    tracing::info!("Address info has been saved.");
                    HttpResponse::Ok().finish()
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to add address {} on network {}: {:?}",
                        form.contract_address,
                        form.network,
                        e
                    );
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to add network {}: {:?}", form.network, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
