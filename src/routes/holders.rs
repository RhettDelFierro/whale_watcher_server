use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
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
    place: i32,
    // TODO: will still need to make a separate table
    amount: BigDecimal,
    // TODO: will still need to make a separate table
    // timestamp: i64, // TODO: will be from Utc::now().timestamp() from a time passed in by the front end.
}

#[tracing::instrument(name = "Saving new network in the database", skip(form, pool))]
pub async fn insert_network(pool: &PgPool, form: &HolderData) -> Result<(), sqlx::Error> {
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
        e
    })?;
    Ok(())
}

#[tracing::instrument(name = "Saving new token name in the database", skip(form, pool))]
pub async fn insert_token_name(pool: &PgPool, form: &HolderData) -> Result<(), sqlx::Error> {
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
        e
    })?;
    Ok(())
}

pub async fn insert_address(
    pool: &PgPool,
    network: &String,
    address: &String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
                INSERT INTO addresses (network_id, address)
                VALUES (
                 (SELECT network_id FROM networks WHERE network_name = $1),
                 $2
                )
                ON CONFLICT DO NOTHING;
                "#,
        network,
        address
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Saving new holder totals details in the database",
    skip(form, pool)
)]
pub async fn insert_holder_totals(pool: &PgPool, form: &HolderData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO holder_totals (network_id, address, token_name_id, place, amount, checked_on)
        VALUES (
            (SELECT network_id FROM networks WHERE network_name = $1),
            $2,
            (SELECT token_name_id FROM token_names WHERE token_name = $3),
            $4,
            $5,
            $6
        );
        "#,
        form.network,
        form.holder_address,
        form.token_name,
        form.place,
        form.amount,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Adding a new holder.",
    skip(form, pool),
    fields(
        network = %form.network,
        token_name = %form.token_name,
        contract_address = %form.contract_address,
        holder_address = %form.holder_address,
        place = %form.place,
        amount = %form.amount,
    )
)]
pub async fn add_holder(form: web::Form<HolderData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_network(&pool, &form).await {
        Ok(_) => match insert_token_name(&pool, &form).await {
            Ok(_) => match insert_address(&pool, &form.network, &form.contract_address).await {
                Ok(_) => match insert_address(&pool, &form.network, &form.holder_address).await {
                    Ok(_) => match insert_holder_totals(&pool, &form).await {
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
