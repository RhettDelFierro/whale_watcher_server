use super::{
    error_chain_fmt, insert_address, insert_network, insert_token_name, BlockchainAppError,
};
use crate::domain::{
    Address, AddressType, HolderDescription, HolderDescriptions, HolderInfo, HolderTotals, Network,
    Notes, TokenName,
};
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
    contract_address: String,
    address_type: String,
    notes: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    network: String,
    holder_descriptions: Vec<HolderData>,
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

impl TryFrom<FormData> for HolderDescriptions {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let mut holder_descriptions = vec![];
        let network = Network::parse(value.network)?;
        for holder in value.holder_descriptions {
            let holder_address = Address::parse(holder.holder_address)?;
            let contract_address = Address::parse(holder.contract_address)?;
            let address_type = AddressType::parse(holder.address_type)?;
            let notes = Notes::parse(holder.notes)?;
            holder_descriptions.push(HolderDescription {
                holder_address,
                contract_address,
                address_type,
                notes,
            })
        }

        Ok(Self {
            network,
            holder_descriptions,
        })
    }
}

pub async fn add_holder_descriptions(
    form: web::Json<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, BlockchainAppError> {
    let holder_descriptions: HolderDescriptions = form
        .0
        .try_into()
        .map_err(BlockchainAppError::ValidationError)?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    insert_network(&mut transaction, &holder_descriptions.network)
        .await
        .context("Failed to insert network in the database.")?;
    for holder in holder_descriptions.holder_descriptions {
        insert_address(
            &mut transaction,
            &holder_descriptions.network,
            &holder.holder_address,
        )
        .await
        .context(format!(
            "Failed to insert contract address {} in the database.",
            &holder.holder_address.as_ref()
        ))?;
        insert_address(
            &mut transaction,
            &holder_descriptions.network,
            &holder.contract_address,
        )
        .await
        .context(format!(
            "Failed to insert contract address {} in the database.",
            &holder.contract_address.as_ref()
        ))?;
    }

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store holder total.")?;
    Ok(HttpResponse::Ok().finish())
}
