mod address;
mod email;
mod holder_total;
mod network;
mod new_subscriber;
mod notes;
mod scam_creator;
mod scam_token;
mod scam_type;
mod subscriber_name;
mod token_name;
mod scammer_query;

pub use address::Address;
pub use email::Email;
pub use holder_total::HolderTotal;
pub use network::Network;
pub use new_subscriber::NewSubscriber;
pub use notes::Notes;
pub use scam_creator::ScamCreator;
pub use scam_token::ScamToken;
pub use scam_type::ScamType;
pub use subscriber_name::SubscriberName;
pub use token_name::TokenName;
pub use scammer_query::ScammerQuery;

const MAX_LIMIT_CHARACTERS: usize = 255;
