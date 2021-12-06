use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct HolderData {
    network: String,
    token_name: String,       // TODO: will still need to make a separate table
    contract_address: String, // TODO: will still need to make a separate table
    holder_address: String,
    place: u16,     // TODO: will still need to make a separate table
    amount: f64,    // TODO: will still need to make a separate table
    timestamp: i64, // TODO: will be from Utc::now().timestamp() from a time passed in by the front end.
}

pub async fn add_holder(_form: web::Form<HolderData>, pool: web::Data<PgPool>) -> HttpResponse {
    tracing::info!("Adding {} to networks.", _form.network);
    match sqlx::query!(
        r#"
        INSERT INTO networks (network_name) VALUES ($1) ON CONFLICT DO NOTHING;
        "#,
        _form.network
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("Adding {} to address info.", _form.contract_address);
            match sqlx::query!(
                r#"
                INSERT INTO addresses (network_id, address)
                VALUES (
                 (SELECT network_id FROM networks WHERE network_name = $1),
                 $2
                );
                "#,
                _form.network,
                _form.contract_address
            )
            .execute(pool.get_ref())
            .await
            {
                Ok(_) => {
                    tracing::info!("Address info has been saved.");
                    HttpResponse::Ok().finish()
                },
                Err(e) => {
                    tracing::error!("Failed to add address {} on network {}: {:?}", _form.contract_address, _form.network, e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to add network {}: {:?}", _form.network, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
