use super::{insert_address, insert_network, insert_token_name, BlockchainAppError};
use crate::domain::{Address, HolderTotal, Network, TokenName};
use actix_web::ResponseError;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::{PgPool, Postgres, Transaction};
use std::convert::{TryFrom, TryInto};
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    network: String,
    token_name: String,
    contract_address: String,
    holder_address: String,
    place: i32,
    amount: BigDecimal,
}

impl TryFrom<FormData> for HolderTotal {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let network = Network::parse(value.network)?;
        let token_name = TokenName::parse(value.token_name)?;
        let contract_address = Address::parse(value.contract_address)?;
        let holder_address = Address::parse(value.holder_address)?;
        let place = value.place;
        let amount = value.amount;
        Ok(Self {
            network,
            token_name,
            contract_address,
            holder_address,
            place,
            amount,
        })
    }
}

#[tracing::instrument(
    name = "Saving new holder totals details in the database",
    skip(transaction, holder_total)
)]
pub async fn insert_holder_totals(
    transaction: &mut Transaction<'_, Postgres>,
    holder_total: &HolderTotal,
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
        holder_total.network.as_ref(),
        holder_total.holder_address.as_ref(),
        holder_total.token_name.as_ref(),
        holder_total.place,
        holder_total.amount,
        Utc::now(),
        holder_total.contract_address.as_ref(),
    )
        .execute(transaction)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}

#[allow(clippy::async_yields_async)]
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
pub async fn add_holder(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let holder_total: HolderTotal = form
        .0
        .try_into()
        .map_err(BlockchainAppError::ValidationError)?;
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    insert_network(&mut transaction, &scam_creator.network_of_scammed_token)
        .await
        .context("Failed to insert network in the database.")?;
    insert_token_name(&mut transaction, &holder_total.token_name)
        .await
        .context(format!(
            "Failed to insert token name {}",
            &holder_total.token_name.as_ref()
        ))?;

    if insert_address(
        &mut transaction,
        &holder_total.network,
        &holder_total.contract_address,
    )
    .await
    .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    if insert_address(
        &mut transaction,
        &holder_total.network,
        &holder_total.holder_address,
    )
    .await
    .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    if insert_holder_totals(&mut transaction, &holder_total)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    if transaction.commit().await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}
#[derive(Debug)]
pub struct StoreHolderTotalError(sqlx::Error);
impl ResponseError for StoreHolderTotalError {}
impl std::fmt::Display for StoreHolderTotalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while \
            trying to store a this holder."
        )
    }
}

#[derive(serde::Deserialize)]
pub struct Parameters {
    network: String,
    contract_address: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HoldersResponse {
    pub data: Vec<FormData>,
}

#[allow(clippy::async_yields_async)]
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
    )
        .fetch_all(pool.get_ref())
        .await {
        Ok(rows) => {
            let mut holders: HoldersResponse = HoldersResponse {
                data: vec![]
            };
            for row in rows {
                let holder = FormData {
                    network: row.network_name,
                    token_name: row.token_name,
                    contract_address: row.contract_address,
                    holder_address: row.holder_address,
                    place: row.place,
                    amount: row.amount,
                };
                holders.data.push(holder);
            };
            HttpResponse::Ok().json(holders)
        }
        Err(e) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}
