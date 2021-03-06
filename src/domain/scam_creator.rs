use super::{Address, Network, Notes, TokenName};

pub struct ScamCreator {
    pub address: Address,
    pub notes: Notes,
    pub network_of_scammed_token: Network,
    pub scammed_contract_address: Address,
}
