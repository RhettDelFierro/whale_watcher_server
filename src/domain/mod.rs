mod address;
mod holder_total;
mod email;
mod network;
mod token_name;

pub use address::Address;
pub use holder_total::HolderTotal;
pub use email::Email;
pub use network::Network;
pub use token_name::TokenName;

const MAX_LIMIT_CHARACTERS: usize = 255;
