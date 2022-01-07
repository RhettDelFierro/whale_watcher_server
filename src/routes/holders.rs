use super::{
    error_chain_fmt, insert_address, insert_network, insert_token_name, BlockchainAppError,
};
use crate::domain::{Address, HolderInfo, HolderTotals, Network, TokenName};
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
pub struct HolderData {
    holder_address: String,
    place: i32,
    amount: BigDecimal,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    network: String,
    token_name: String,
    contract_address: String,
    holders: Vec<HolderData>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HolderRowData {
    network: String,
    token_name: String,
    contract_address: String,
    holder_address: String,
    place: i32,
    amount: BigDecimal,
    checked_on: DateTime<Utc>,
}

impl TryFrom<FormData> for HolderTotals {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let network = Network::parse(value.network)?;
        let token_name = TokenName::parse(value.token_name)?;
        let contract_address = Address::parse(value.contract_address)?;
        let mut holders = vec![];
        for holder in value.holders {
            let holder_address = Address::parse(holder.holder_address)?;
            let place = holder.place;
            let amount = holder.amount;
            holders.push(HolderInfo {
                holder_address,
                place,
                amount,
            })
        }

        Ok(Self {
            network,
            token_name,
            contract_address,
            holders,
        })
    }
}

#[tracing::instrument(
    name = "Saving new holder totals details in the database",
    skip(transaction, network_name, token_name, contract_address, holder_info)
)]
pub async fn insert_holder_totals(
    transaction: &mut Transaction<'_, Postgres>,
    network_name: &str,
    token_name: &str,
    contract_address: &str,
    holder_info: &HolderInfo,
) -> Result<(), StoreHolderTotalError> {
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
        network_name,
        holder_info.holder_address.as_ref(),
        token_name,
        holder_info.place,
        holder_info.amount,
        Utc::now(),
        contract_address,
    )
        .execute(transaction)
        .await
        .map_err(|e| { StoreHolderTotalError(e) })?;
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
    )
)]
pub async fn add_holders(
    form: web::Json<FormData>, //web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, BlockchainAppError> {
    let holder_total: HolderTotals = form
        .0
        .try_into()
        .map_err(BlockchainAppError::ValidationError)?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    insert_network(&mut transaction, &holder_total.network)
        .await
        .context("Failed to insert network in the database.")?;

    insert_token_name(&mut transaction, &holder_total.token_name)
        .await
        .context(format!(
            "Failed to insert token name {}",
            &holder_total.token_name.as_ref()
        ))?;
    insert_address(
        &mut transaction,
        &holder_total.network,
        &holder_total.contract_address,
    )
    .await
    .context(format!(
        "Failed to insert contract address {} in the database.",
        &holder_total.contract_address.as_ref()
    ))?;
    for holder in holder_total.holders {
        insert_address(
            &mut transaction,
            &holder_total.network,
            &holder.holder_address,
        )
        .await
        .context(format!(
            "Failed to insert holder {} in the database.",
            &holder.holder_address.as_ref()
        ))?;

        insert_holder_totals(
            &mut transaction,
            holder_total.network.as_ref(),
            holder_total.token_name.as_ref(),
            holder_total.contract_address.as_ref(),
            &holder,
        )
        .await
        .context(format!(
            "Failed to insert holder {} and contract address {} in the database.",
            &holder.holder_address.as_ref(),
            &holder_total.contract_address.as_ref()
        ))?;
    }
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store holder total.")?;

    Ok(HttpResponse::Ok().finish())
}
pub struct StoreHolderTotalError(sqlx::Error);

impl std::error::Error for StoreHolderTotalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Debug for StoreHolderTotalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::fmt::Display for StoreHolderTotalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database failure was encountered while trying to store a scammer."
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
    pub data: Vec<HolderRowData>,
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

                let holder = HolderRowData {
                    network: row.network_name,
                    token_name: row.token_name,
                    contract_address: row.contract_address,
                    holder_address: row.holder_address,
                    place: row.place,
                    amount: row.amount,
                    checked_on: row.checked_on
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
