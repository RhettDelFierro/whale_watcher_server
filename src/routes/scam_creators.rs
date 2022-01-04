use super::{insert_address, insert_network, BlockchainAppError};
use crate::domain::{Address, Network, Notes, ScamCreator, ScamType, TokenCreatorQuery};
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
pub struct FormDataScammers {
    address: String,
    notes: Option<String>,
    network_of_scammed_token: String,
    scammed_contract_address: String,
}

impl TryFrom<FormDataScammers> for ScamCreator {
    type Error = String;

    fn try_from(value: FormDataScammers) -> Result<Self, Self::Error> {
        let address = Address::parse(value.address)?;
        let notes = Notes::parse(value.notes)?;
        let network_of_scammed_token = Network::parse(value.network_of_scammed_token)?;
        let scammed_contract_address = Address::parse(value.scammed_contract_address)?;
        Ok(Self {
            address,
            notes,
            network_of_scammed_token,
            scammed_contract_address,
        })
    }
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(name = "Inserting a scammer.", skip(transaction, scammer))]
pub async fn insert_scammer(
    transaction: &mut Transaction<'_, Postgres>,
    scammer: &ScamCreator,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO scam_token_creators (address, notes, network_of_scammed_token, scammed_contract_address)
        VALUES (
            $1,
            $2,
            (SELECT network_id FROM networks WHERE network_name = $3),
            $4
        );
        "#,
        scammer.address.as_ref(),
        scammer.notes.as_ref(),
        scammer.network_of_scammed_token.as_ref(),
        scammer.scammed_contract_address.as_ref(),
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
    name = "Adding a new scammmer.",
    skip(form, pool),
    fields(
        address = %form.address,
        network_of_scammed_token = %form.network_of_scammed_token,
        scammed_contract_address = %form.scammed_contract_address
    )
)]
pub async fn register_scammer(
    form: web::Form<FormDataScammers>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let scam_creator: ScamCreator = form
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

    insert_address(
        &mut transaction,
        &scam_creator.network_of_scammed_token,
        &scam_creator.address,
    )
    .await
    .context(format!(
        "Failed to insert new scammer's address {} in the database.",
        &scam_creator.address.as_ref()
    ))?;

    insert_address(
        &mut transaction,
        &scam_creator.network_of_scammed_token,
        &scam_creator.scammed_contract_address,
    )
    .await
    .context(format!(
        "Failed to insert scammed contract address {} in the database.",
        &scam_creator.scammed_contract_address.as_ref()
    ))?;

    insert_scammer(&mut transaction, &scam_creator)
        .await
        .context(format!(
            "Failed to insert scammer {} with contract address {} in the database.",
            &scam_creator.address.as_ref(),
            &scam_creator.scammed_contract_address.as_ref()
        ))?;
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a scammer.")?;
    HttpResponse::Ok().finish()
}

#[derive(Debug)]
pub struct StoreScamCreatorError(sqlx::Error);
impl ResponseError for StoreScamCreatorError {}
impl std::fmt::Display for StoreScamCreatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while \
            trying to store a this scam creator."
        )
    }
}

#[derive(serde::Deserialize)]
pub struct ScammerParameters {
    token_creator_address: String,
}

impl TryFrom<ScammerParameters> for TokenCreatorQuery {
    type Error = String;

    fn try_from(value: ScammerParameters) -> Result<Self, Self::Error> {
        let token_creator_address = Address::parse(value.token_creator_address)?;
        Ok(Self {
            token_creator_address,
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ScamTokenCreatorResponse {
    pub data: Vec<FormDataScammers>,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    name = "Getting a scammmer.",
    skip(pool, parameters),
    fields(
        token_creator_address = %parameters.token_creator_address
    )
)]
pub async fn get_scammers(
    parameters: web::Query<ScammerParameters>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let scammer_query: TokenCreatorQuery = match parameters.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match sqlx::query!(
        r#"
        SELECT s.address, s.notes, n.network_name, s.scammed_contract_address FROM scam_token_creators s
        INNER JOIN networks n
            ON s.network_of_scammed_token = n.network_id
        WHERE s.address = $1
        ;
        "#,
        scammer_query.token_creator_address.as_ref(),
    )
        .fetch_all(pool.get_ref())
        .await {
        Ok(rows) => {
            let mut scammers = ScamTokenCreatorResponse {
                data: vec![]
            };
            for row in rows {
                let scammer = FormDataScammers {
                    address: row.address,
                    notes: row.notes,
                    network_of_scammed_token: row.network_name,
                    scammed_contract_address: row.scammed_contract_address
                };
                scammers.data.push(scammer);
            };
            HttpResponse::Ok().json(scammers)
        }
        Err(e) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}
