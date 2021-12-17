use super::{insert_address, insert_network};
use crate::domain::{Address, Network, Notes, ScamCreator, ScamType};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::{PgPool, Postgres, Transaction};
use std::convert::{TryFrom, TryInto};
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    address: String,
    notes: String,
    scam_creator_network: String,
    scam_creator_address: String,
    scam_type: String,
}

pub async fn register_scam_token(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
