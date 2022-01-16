use super::{Address, AddressType, Network, Notes};

pub struct HolderDescriptions {
    pub network: Network,
    pub holder_descriptions: Vec<HolderDescription>,
}

pub struct HolderDescription {
    pub holder_address: Address,
    pub address_types: Vec<AddressType>,
    pub contract_address: Address,
    pub notes: Notes,
}
