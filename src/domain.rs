use unicode_segmentation::UnicodeSegmentation;
use sqlx::Type;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum Network {
    ETH,
    BSC,
    ADA,
    AVAX,
    MATIC,
    FTM,
    SOL,
    LUNA,
    DOT,
    MOVR,
}

// can probably have this return an enum for network.
fn derive_network(s: &str) -> Option<Network> {
    let str = s.to_lowercase();
    if str == "ethereum" || str == "eth" {
        return Some(Network::ETH);
    } else if str == "binancesmartchain" || str == "binance" || str == "bsc" {
        return Some(Network::BSC);
    } else if str == "cardano" || str == "ada" {
        return Some(Network::ADA);
    } else if str == "avalanche" || str == "avax" {
        return Some(Network::AVAX);
    } else if str == "polygon" || str == "matic" {
        return Some(Network::MATIC);
    } else if str == "fantom" || str == "ftm" {
        return Some(Network::FTM);
    } else if str == "solana" || str == "sol" {
        return Some(Network::SOL);
    } else if str == "terra" || str == "terraluna" || str == "luna" {
        return Some(Network::LUNA);
    } else if str == "polkadot" || str == "dot" {
        return Some(Network::DOT);
    } else if str == "moonriver" || str == "movr" {
        return Some(Network::MOVR);
    }
    None
}

impl Network {
    pub fn parse(s: String) -> Network {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        let network = derive_network(&s);
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters || network.is_none() {
            panic!("{}", format!("{} is not a valid network name.", s))
        } else {
            network.unwrap()
        }
    }
}

impl AsRef<str> for Network {
    fn as_ref(&self) -> &str {
        match self {
            Network::ETH => "eth",
            Network::BSC => "bsc",
            Network::ADA => "ada",
            Network::AVAX => "avax",
            Network::MATIC => "matic",
            Network::FTM => "ftm",
            Network::SOL => "sol",
            Network::LUNA => "luna",
            Network::DOT => "dot",
            Network::MOVR => "movr",
        }
    }
}

pub struct Address(String);

impl Address {
    pub fn parse(s: String) -> Address {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!("{}", format!("{} is not a valid address.", &s))
        } else {
            Self(s)
        }
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct AddressInfo {
    pub network: Network,
    pub address: Address,
}