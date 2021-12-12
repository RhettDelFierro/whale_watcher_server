use super::{Address, Network, TokenName};
use sqlx::types::BigDecimal;

pub struct HolderTotal {
    pub network: Network,
    pub token_name: TokenName,
    pub contract_address: Address,
    pub holder_address: Address,
    pub place: i32,
    pub amount: BigDecimal,
}
