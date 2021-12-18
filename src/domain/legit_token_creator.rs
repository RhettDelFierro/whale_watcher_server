use super::{Address, Network, Notes, TokenName};

pub struct LegitTokenCreator {
    pub address: Address,
    pub notes: Notes,
    pub network_of_legit_token: Network,
    pub legit_contract_address: Address,
}
