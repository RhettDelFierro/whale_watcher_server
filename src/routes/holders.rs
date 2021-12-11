use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;
use crate::domain::{Address, Network, AddressInfo};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HolderData {
    network: String,
    token_name: String,
    contract_address: String,
    holder_address: String,
    place: i32,
    amount: BigDecimal,
}

#[tracing::instrument(name = "Saving new network in the database", skip(address_info, pool))]
pub async fn insert_network(pool: &PgPool, address_info: &AddressInfo) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO networks (network_name) VALUES ($1) ON CONFLICT DO NOTHING;
        "#,
        address_info.network.as_ref()
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

#[tracing::instrument(name = "Saving new address in the database", skip(address_info, pool))]
pub async fn insert_address(
    pool: &PgPool,
    address_info: &AddressInfo,
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
        address_info.network.as_ref(),
        address_info.address.as_ref()
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
skip(form, pool, holder_address_info, contract_address_info)
)]
pub async fn insert_holder_totals(
    pool: &PgPool,
    holder_address_info: &AddressInfo,
    contract_address_info: &AddressInfo,
    form: &HolderData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO holder_totals (network_id, holder_address, token_name_id, place, amount, checked_on, contract_address)
        VALUES (
            (SELECT network_id FROM networks WHERE network_name = $1),
            $2,
            (SELECT token_name_id FROM token_names WHERE token_name = $3),
            $4,
            $5,
            $6,
            $7
        );
        "#,
        holder_address_info.network.as_ref(),
        holder_address_info.address.as_ref(),
        form.token_name,
        form.place,
        form.amount,
        Utc::now(),
        contract_address_info.address.as_ref(),
    )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct Parameters {
    network: String,
    contract_address: String,
}

#[tracing::instrument(
name = "Adding a new holder.",
skip(form, pool),
fields(
network = % form.network,
token_name = % form.token_name,
contract_address = % form.contract_address,
holder_address = % form.holder_address,
place = % form.place,
amount = % form.amount,
)
)]
pub async fn add_holder(form: web::Form<HolderData>, pool: web::Data<PgPool>) -> HttpResponse {
    let contract_address_info = AddressInfo {
        network: Network::parse(form.0.network.clone()).expect("Network name is invalid."),
        address: Address::parse(String::from(&form.0.contract_address[..])).expect("contract_address format is invalid"),
    };
    let holder_address_info = AddressInfo {
        network: Network::parse(form.0.network.clone()).expect("Network name is invalid."),
        address: Address::parse(String::from(&form.0.holder_address[..])).expect("holder_address format is invalid"),
    };
    match insert_network(&pool, &holder_address_info).await {
        Ok(_) => match insert_token_name(&pool, &form).await {
            Ok(_) => match insert_address(&pool, &contract_address_info).await {
                Ok(_) => match insert_address(&pool, &holder_address_info).await {
                    Ok(_) => match insert_holder_totals(&pool, &holder_address_info, &contract_address_info, &form).await {
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HoldersResponse {
    pub data: Vec<HolderData>,
}

#[tracing::instrument(
name = "Fetching holders.",
skip(parameters, pool),
fields(
network = % parameters.network,
contract_address = % parameters.contract_address
)
)]
pub async fn get_holder(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
        SELECT h.*, t.token_name, n.network_name FROM holder_totals h
        INNER JOIN token_names t
            ON h.token_name_id = t.token_name_id
        INNER JOIN addresses a
            ON a.address = h.holder_address AND a.network_id = h.network_id AND h.contract_address = $2
        INNER JOIN networks n
            ON n.network_id = h.network_id AND n.network_name = $1
        ORDER BY h.checked_on ASC;
        ;
        "#,
        parameters.network,
        parameters.contract_address,
    ).fetch_all(pool.get_ref())
        .await {
        Ok(rows) => {
            let mut holders: Vec<HolderData> = vec![];
            for row in rows {
                let holder = HolderData {
                    network: row.network_name,
                    token_name: row.token_name,
                    contract_address: row.contract_address,
                    holder_address: row.holder_address,
                    place: row.place,
                    amount: row.amount,
                };
                holders.push(holder);
            };
            let response = HoldersResponse {
                data: holders
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}