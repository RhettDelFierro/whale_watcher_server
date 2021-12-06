use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct HolderData {
    network: String,
    token_name: String,
    contract_address: String,
    // TODO: will still need to make a separate table
    holder_address: String,
    place: u16,
    // TODO: will still need to make a separate table
    amount: f64,
    // TODO: will still need to make a separate table
    timestamp: i64, // TODO: will be from Utc::now().timestamp() from a time passed in by the front end.
}

pub async fn insert_network(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO networks (network_name) VALUES ($1) ON CONFLICT DO NOTHING;
        "#,
        form.network
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
    })?;
    Ok(())
}

pub async fn insert_token_name(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO token_names (token_name) VALUES ($1) ON CONFLICT DO NOTHING;
        "#,
        form.token_name
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
    })?;
    Ok(())
}

pub async fn insert_address(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
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
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
    })?;
    Ok(())
}

pub async fn insert_place_amount(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO addresses (network_name) VALUES ($1) ON CONFLICT DO NOTHING;
        "#,
        form.network
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Adding a new holder.",
    skip(form, pool),
        fields(
        network = % form.network,
        contract_address = % form.contract_address
    )
)]
pub async fn add_holder(form: web::Form<HolderData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_network(&pool, &form).await {
        Ok(_) => match insert_token_name(&pool, &form).await {
            Ok(_) => match insert_address(&pool, &form).await {
                Ok(_) => match insert_address(&pool, &form).await {
                    Ok(_) => match insert_place_amount(&pool, &form).await {
                        Ok(_) => HttpResponse::Ok().finish(),
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    },
                    Err(_) => HttpResponse::InternalServerError().finish(),
                },
                Err(_) => HttpResponse::InternalServerError().finish(),
            },
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
