use super::{Address, Network, Notes, TokenName};
use sqlx::types::BigDecimal;

pub struct LegitTokenCreator {
    pub address: Address,
    pub notes: Notes,
    pub network_of_major_token: Network,
    pub big_contract_address: Address,
}
