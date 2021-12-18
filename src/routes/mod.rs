mod health_check;
mod holders;
mod legit_token_creator;
mod scam_creators;
mod scam_tokens;
mod subscriptions;
mod subscriptions_confirm;

pub use health_check::*;
pub use holders::*;
pub use legit_token_creator::*;
pub use scam_creators::*;
pub use scam_tokens::*;
pub use subscriptions::*;
pub use subscriptions_confirm::*;

use crate::domain::{Address, HolderTotal, Network, TokenName};
use sqlx::{PgPool, Postgres, Transaction};
use tracing_futures::Instrument;

#[tracing::instrument(
    name = "Saving new network in the database",
    skip(network, transaction)
)]
async fn insert_network(
    transaction: &mut Transaction<'_, Postgres>,
    network: &Network,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO networks (network_name) VALUES ($1) ON CONFLICT DO NOTHING;
        "#,
        network.as_ref()
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Saving new token name in the database",
    skip(token_name, transaction)
)]
async fn insert_token_name(
    transaction: &mut Transaction<'_, Postgres>,
    token_name: &TokenName,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO token_names (token_name) VALUES ($1) ON CONFLICT DO NOTHING;
        "#,
        token_name.as_ref()
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Saving new address in the database",
    skip(network, address, transaction)
)]
async fn insert_address(
    transaction: &mut Transaction<'_, Postgres>,
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
    .execute(transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
