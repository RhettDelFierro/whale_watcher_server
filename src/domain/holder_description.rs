use super::{Address, AddressType, Network, Notes};

pub struct HolderDescription {
    pub network: Network,
    pub holder_address: Address,
    pub address_type: AddressType,
    pub contract_address: Address,
    pub notes: Notes,
}
