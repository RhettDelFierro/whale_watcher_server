use crate::domain::{Address, Network, Notes, ScamType, ScamCreator};
use super::{insert_network,insert_address};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use std::convert::{TryFrom, TryInto};
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    address: String,
    notes: String,
    network_of_scammed_token: String,
    scammed_contract_address: String,
}

impl TryFrom<FormData> for ScamCreator {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let address = Address::parse(value.address)?;
        let notes = Notes::parse(value.notes)?;
        let network_of_scammed_token = Network::parse(value.network_of_scammed_token)?;
        let scammed_contract_address = Address::parse(value.scammed_contract_address)?;
        Ok(Self {
            address,
            notes,
            network_of_scammed_token,
            scammed_contract_address
        })
    }
}

pub async fn register_scammer(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}