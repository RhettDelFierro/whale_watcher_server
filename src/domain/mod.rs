mod address;
mod email;
mod holder_totals;
mod legit_token_creator;
mod network;
mod new_subscriber;
mod notes;
mod scam_creator;
mod scam_type;
mod subscriber_name;
mod token_creator_query;
mod token_name;

pub use address::Address;
pub use email::Email;
pub use holder_totals::{HolderInfo, HolderTotals};
pub use legit_token_creator::LegitTokenCreator;
pub use network::Network;
pub use new_subscriber::NewSubscriber;
pub use notes::Notes;
pub use scam_creator::ScamCreator;
pub use scam_type::ScamType;
pub use subscriber_name::SubscriberName;
pub use token_creator_query::TokenCreatorQuery;
pub use token_name::TokenName;

const MAX_LIMIT_CHARACTERS: usize = 255;
