use super::{
    error_chain_fmt, insert_address, insert_network, insert_token_name, BlockchainAppError,
};
use crate::domain::{Address, AddressType, HolderDescription, HolderDescriptions, Network, Notes};
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
    address_types: Vec<String>,
    notes: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    network_name: String,
    holder_descriptions: Vec<HolderData>,
}

impl TryFrom<FormData> for HolderDescriptions {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let mut holder_descriptions = vec![];
        let network = Network::parse(value.network_name)?;
        for holder in value.holder_descriptions {
            let mut address_types = vec![];
            let holder_address = Address::parse(holder.holder_address)?;
            let contract_address = Address::parse(holder.contract_address)?;
            for address_type in holder.address_types {
                let at = AddressType::parse(address_type)?;
                address_types.push(at)
            }
            let notes = Notes::parse(holder.notes)?;

            holder_descriptions.push(HolderDescription {
                holder_address,
                contract_address,
                address_types,
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
        insert_holder_description(
            &mut transaction,
            &holder_descriptions.network.as_ref(),
            &holder,
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

#[tracing::instrument(
    name = "Saving new holder totals details in the database",
    skip(transaction, network_name, holder_description)
)]
pub async fn insert_holder_description(
    transaction: &mut Transaction<'_, Postgres>,
    network_name: &str,
    holder_description: &HolderDescription,
) -> Result<(), StoreHolderDescriptionError> {
    let address_types = holder_description
        .address_types
        .iter()
        .map(|hd| hd.as_ref().to_string())
        .collect::<Vec<String>>();
    sqlx::query!(
        r#"
        INSERT INTO holder_descriptions (network_id, holder_address, contract_address, notes, address_types)
        VALUES (
            (SELECT network_id FROM networks WHERE network_name = $1),
            $2,
            $3,
            $4,
            $5
        );
        "#,
        network_name,
        holder_description.holder_address.as_ref(),
        holder_description.contract_address.as_ref(),
        holder_description.notes.as_ref(),
        &address_types[..],
    )
        .execute(transaction)
        .await
        .map_err(|e| { StoreHolderDescriptionError(e) })?;
    Ok(())
}

pub struct StoreHolderDescriptionError(sqlx::Error);

impl std::error::Error for StoreHolderDescriptionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Debug for StoreHolderDescriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::fmt::Display for StoreHolderDescriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database failure was encountered while trying to store a scammer."
        )
    }
}

#[derive(serde::Deserialize)]
pub struct Parameters {
    holder_addresses: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HolderRowData {
    network_name: String,
    contract_address: String,
    holder_address: String,
    notes: Option<String>,
    address_types: Option<Vec<String>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HolderDescriptionsResponse {
    pub data: Vec<HolderRowData>,
}

pub async fn get_holder_descriptions(
    form: web::Json<Parameters>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let mut holders: HolderDescriptionsResponse = HolderDescriptionsResponse { data: vec![] };
    for holder_address in &form.holder_addresses {
        let holder_descriptions =
            match get_holder_description_from_holder_address(&pool, holder_address.to_string())
                .await
            {
                Ok(holder_descriptions) => holder_descriptions,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };
        for hd in holder_descriptions {
            holders.data.push(hd)
        }
    }
    HttpResponse::Ok().json(holders)
}

#[tracing::instrument(name = "Get holder from holder_address", skip(holder_address, pool))]
pub async fn get_holder_description_from_holder_address(
    pool: &PgPool,
    holder_address: String,
) -> Result<Vec<HolderRowData>, sqlx::Error> {
    let results = sqlx::query!(
        r#"
        SELECT h.*, n.network_name FROM holder_descriptions h
        INNER JOIN addresses a
            ON a.address = h.holder_address AND a.network_id = h.network_id AND h.holder_address = $1
        INNER JOIN networks n
            ON n.network_id = h.network_id
        ;
        "#,
        holder_address,
    )
        .fetch_all(pool)
        .await?;
    let holder_descriptions = results
        .into_iter()
        .map(|r| HolderRowData {
            network_name: r.network_name,
            contract_address: r.contract_address,
            holder_address: r.holder_address,
            notes: r.notes,
            address_types: r.address_types,
        })
        .collect();
    Ok(holder_descriptions)
}
