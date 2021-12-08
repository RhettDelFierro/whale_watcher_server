use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
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
        form.network,
        form.holder_address,
        form.token_name,
        form.place,
        form.amount,
        Utc::now(),
        form.contract_address,
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HoldersResponse {
    data: Vec<HolderData>
}

// #[derive(serde::Deserialize, serde::Serialize)]
// pub struct HolderResponse {
//     network: String,
//     token_name: String,
//     contract_address: String,
//     holder_address: String,
//     place: i32,
//     amount: BigDecimal,
// }

#[tracing::instrument(
name = "Fetching holders.",
skip(parameters, pool),
fields(
network = % parameters.network,
contract_address = % parameters.contract_address
)
)]
pub async fn get_holder(parameters: web::Query<Parameters>, pool: web::Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query!(
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
        .await
        .expect("Failed to fetch holders.");
    println!("{:?}", rows);
    let mut holders: Vec<HolderData> = vec![];
    for row in rows {
        let holder = HolderData {
            network: row.network_name,
            token_name: row.token_name,
            contract_address: row.contract_address.unwrap(),
            // TODO: will still need to make a separate table
            holder_address: row.holder_address,
            place: row.place,
            // TODO: will still need to make a separate table
            amount: row.amount,
        };
        holders.push(holder);
    };
    let response = HoldersResponse {
        data: holders
    };
    HttpResponse::Ok().json(response)
}
