use super::{insert_address, insert_network};
use crate::domain::{Address, LegitTokenCreator, Network, Notes, ScamType, TokenCreatorQuery};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::{PgPool, Postgres, Transaction};
use std::convert::{TryFrom, TryInto};
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormDataLegitTokenCreator {
    address: String,
    notes: Option<String>,
    network_of_legit_token: String,
    legit_contract_address: String,
}

impl TryFrom<FormDataLegitTokenCreator> for LegitTokenCreator {
    type Error = String;

    fn try_from(value: FormDataLegitTokenCreator) -> Result<Self, Self::Error> {
        let address = Address::parse(value.address)?;
        let notes = Notes::parse(value.notes)?;
        let network_of_legit_token = Network::parse(value.network_of_legit_token)?;
        let legit_contract_address = Address::parse(value.legit_contract_address)?;
        Ok(Self {
            address,
            notes,
            network_of_legit_token,
            legit_contract_address,
        })
    }
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    name = "Inserting a legit token creator.",
    skip(transaction, legit_token_creator)
)]
pub async fn insert_legit_token_creator(
    transaction: &mut Transaction<'_, Postgres>,
    legit_token_creator: &LegitTokenCreator,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO legit_token_creators (address, notes, network_of_legit_token, legit_contract_address)
        VALUES (
            $1,
            $2,
            (SELECT network_id FROM networks WHERE network_name = $3),
            $4
        );
        "#,
        legit_token_creator.address.as_ref(),
        legit_token_creator.notes.as_ref(),
        legit_token_creator.network_of_legit_token.as_ref(),
        legit_token_creator.legit_contract_address.as_ref(),
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
name = "Adding a new legit token creator.",
skip(form, pool),
fields(
address = %form.address,
network_of_legit_token = %form.network_of_legit_token,
legit_contract_address = %form.legit_contract_address
)
)]
pub async fn register_legit_token_creator(
    form: web::Form<FormDataLegitTokenCreator>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let legit_token_creator: LegitTokenCreator = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if insert_network(
        &mut transaction,
        &legit_token_creator.network_of_legit_token,
    )
    .await
    .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    if insert_address(
        &mut transaction,
        &legit_token_creator.network_of_legit_token,
        &legit_token_creator.address,
    )
    .await
    .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    if insert_address(
        &mut transaction,
        &legit_token_creator.network_of_legit_token,
        &legit_token_creator.legit_contract_address,
    )
    .await
    .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    if insert_legit_token_creator(&mut transaction, &legit_token_creator)
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

#[derive(serde::Deserialize)]
pub struct LegitTokenCreatorParameters {
    token_creator_address: String,
}

impl TryFrom<LegitTokenCreatorParameters> for TokenCreatorQuery {
    type Error = String;

    fn try_from(value: LegitTokenCreatorParameters) -> Result<Self, Self::Error> {
        let token_creator_address = Address::parse(value.token_creator_address)?;
        Ok(Self {
            token_creator_address,
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LegitTokenCreatorResponse {
    pub data: Vec<FormDataLegitTokenCreator>,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
name = "Getting a legit token creator.",
skip(pool, parameters),
fields(
token_creator_address = %parameters.token_creator_address
)
)]
pub async fn get_legit_token_creators(
    parameters: web::Query<LegitTokenCreatorParameters>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let token_creator_query: TokenCreatorQuery = match parameters.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match sqlx::query!(
        r#"
        SELECT l.address, l.notes, n.network_name, l.legit_contract_address FROM legit_token_creators l
        INNER JOIN networks n
            ON l.network_of_legit_token = n.network_id
        WHERE l.address = $1
        ;
        "#,
        token_creator_query.token_creator_address.as_ref(),
    )
        .fetch_all(pool.get_ref())
        .await {
        Ok(rows) => {
            let mut legit_token_creators = LegitTokenCreatorResponse {
                data: vec![]
            };
            for row in rows {
                let legit_token_creator = FormDataLegitTokenCreator {
                    address: row.address,
                    notes: row.notes,
                    network_of_legit_token: row.network_name,
                    legit_contract_address: row.legit_contract_address
                };
                legit_token_creators.data.push(legit_token_creator);
            };
            HttpResponse::Ok().json(legit_token_creators)
        }
        Err(e) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}
