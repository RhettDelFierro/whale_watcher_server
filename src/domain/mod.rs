mod address;
mod address_info;
mod email;
mod network;
mod token_name;

pub use address::Address;
pub use address_info::AddressInfo;
pub use email::Email;
pub use network::Network;
pub use token_name::TokenName;

const MAX_LIMIT_CHARACTERS: usize = 255;
