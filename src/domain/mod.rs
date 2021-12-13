mod address;
mod email;
mod holder_total;
mod network;
mod token_name;
mod new_subscriber;
mod subscriber_name;

pub use address::Address;
pub use email::Email;
pub use holder_total::HolderTotal;
pub use network::Network;
pub use token_name::TokenName;
pub use new_subscriber::NewSubscriber;
pub use subscriber_name::SubscriberName;

const MAX_LIMIT_CHARACTERS: usize = 255;
