use super::{insert_address, insert_network};
use crate::domain::{Address, Network, Notes, ScamCreator, ScamType, ScammerQuery};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::PgPool;
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

pub async fn insert_scammer(pool: &PgPool, scammer: &ScamCreator)-> Result<(), sqlx::Error> {
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
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}

pub async fn register_scammer(form: web::Form<FormDataScammers>, pool: web::Data<PgPool>) -> HttpResponse {
    let scam_creator: ScamCreator = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match insert_network(&pool, &scam_creator.network_of_scammed_token).await {
            Ok(_) => {
                match insert_address(&pool, &scam_creator.network_of_scammed_token, &scam_creator.address).await {
                    Ok(_) => match insert_address(
                        &pool,
                        &scam_creator.network_of_scammed_token,
                        &scam_creator.scammed_contract_address,
                    )
                        .await
                    {
                        Ok(_) => match insert_scammer(&pool, &scam_creator).await {
                            Ok(_) => HttpResponse::Ok().finish(),
                            Err(_) => HttpResponse::InternalServerError().finish(),
                        },
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    },
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            },
            Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Deserialize)]
pub struct ScammerParameters {
    network: String,
    scammer_address: String,
}

impl TryFrom<ScammerParameters> for ScammerQuery {
    type Error = String;

    fn try_from(value: ScammerParameters) -> Result<Self, Self::Error> {
        let network = Network::parse(value.network)?;
        let scammer_address = Address::parse(value.scammer_address)?;
        Ok(Self {
            network,
            scammer_address,
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ScamTokenResponse {
    pub data: Vec<FormDataScammers>,
}

pub async fn get_scammers(
    parameters: web::Query<ScammerParameters>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let scammer_query: ScammerQuery = match parameters.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match sqlx::query!(
        r#"
        SELECT s.address, s.notes, n.network_name, s.scammed_contract_address FROM scam_token_creators s
        INNER JOIN networks n
            ON s.network_of_scammed_token = n.network_id AND n.network_name = $1
        WHERE s.address = $2
        ;
        "#,
        scammer_query.network.as_ref(),
        scammer_query.scammer_address.as_ref(),
    )
        .fetch_all(pool.get_ref())
        .await {
        Ok(rows) => {
            let mut scammers: ScamTokenResponse = ScamTokenResponse {
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
