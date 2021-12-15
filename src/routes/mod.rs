mod health_check;
mod holders;
mod scam_creators;
mod scam_tokens;
mod subscriptions;

pub use health_check::*;
pub use holders::*;
pub use scam_creators::*;
pub use scam_tokens::*;
pub use subscriptions::*;

use crate::domain::{Address, HolderTotal, Network, TokenName};
use sqlx::PgPool;
use tracing_futures::Instrument;

#[tracing::instrument(name = "Saving new network in the database", skip(network, pool))]
async fn insert_network(pool: &PgPool, network: &Network) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO networks (network_name) VALUES ($1) ON CONFLICT DO NOTHING;
        "#,
        network.as_ref()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(name = "Saving new token name in the database", skip(token_name, pool))]
async fn insert_token_name(pool: &PgPool, token_name: &TokenName) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO token_names (token_name) VALUES ($1) ON CONFLICT DO NOTHING;
        "#,
        token_name.as_ref()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Saving new address in the database",
    skip(network, address, pool)
)]
async fn insert_address(
    pool: &PgPool,
    network: &Network,
    address: &Address,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
                INSERT INTO addresses (network_id, address)
                VALUES (
                 (SELECT network_id FROM networks WHERE network_name = $1),
                 $2
                )
                ON CONFLICT DO NOTHING;
                "#,
        network.as_ref(),
        address.as_ref()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
