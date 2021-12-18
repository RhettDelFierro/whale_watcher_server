use super::{Address, Network};

pub struct TokenCreatorQuery {
    pub network: Network,
    pub token_creator_address: Address,
}
