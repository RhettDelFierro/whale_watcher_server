use super::{
    error_chain_fmt, insert_address, insert_network, insert_token_name, BlockchainAppError,
};
use crate::domain::{Address, AddressType, HolderInfo, HolderTotals, Network, TokenName};
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
    address_type: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    network: String,
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

impl TryFrom<FormData> for HolderDescription {
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
