mod address;
mod address_info;
mod email;
mod network;

pub use address::Address;
pub use address_info::AddressInfo;
pub use email::Email;
pub use network::Network;

const MAX_LIMIT_CHARACTERS: usize = 255;