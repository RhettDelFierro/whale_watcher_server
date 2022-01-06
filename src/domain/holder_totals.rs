use super::{Address, Network, TokenName};
use sqlx::types::BigDecimal;

pub struct HolderTotals {
    pub network: Network,
    pub token_name: TokenName,
    pub contract_address: Address,
    pub holders: Vec<HolderInfo>,
}

pub struct HolderInfo {
    pub holder_address: Address,
    pub place: i32,
    pub amount: BigDecimal,
}
