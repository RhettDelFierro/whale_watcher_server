use super::{Address, Network, Notes, ScamType, TokenName};
use sqlx::types::BigDecimal;

pub struct ScamToken {
    pub address: Address,
    pub notes: Notes,
    pub scam_creator_network: Network,
    pub scam_creator_address: Address,
    pub scam_type: ScamType,
}
