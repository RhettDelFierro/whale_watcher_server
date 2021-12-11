use unicode_segmentation::UnicodeSegmentation;
use sqlx::Type;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug)]
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
fn derive_network(s: &str) -> Result<Network, String> {
    let str = s.to_lowercase();
    if str == "ethereum" || str == "eth" {
        return Ok(Network::ETH);
    } else if str == "binancesmartchain" || str == "binance" || str == "bsc" {
        return Ok(Network::BSC);
    } else if str == "cardano" || str == "ada" {
        return Ok(Network::ADA);
    } else if str == "avalanche" || str == "avax" {
        return Ok(Network::AVAX);
    } else if str == "polygon" || str == "matic" {
        return Ok(Network::MATIC);
    } else if str == "fantom" || str == "ftm" {
        return Ok(Network::FTM);
    } else if str == "solana" || str == "sol" {
        return Ok(Network::SOL);
    } else if str == "terra" || str == "terraluna" || str == "luna" {
        return Ok(Network::LUNA);
    } else if str == "polkadot" || str == "dot" {
        return Ok(Network::DOT);
    } else if str == "moonriver" || str == "movr" {
        return Ok(Network::MOVR);
    }
    Err(String::from("network not supported"))
}

impl Network {
    pub fn parse(s: String) -> Result<Network, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        let network = derive_network(&s);
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters || network.is_err() {
            panic!("{}", format!("{} is not a valid network name.", s))
        } else {
            network
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

#[derive(Debug)]
pub struct Address(String);

impl Address {
    pub fn parse(s: String) -> Result<Address, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!("{}", format!("{} is not a valid address.", &s))
        } else {
            Ok(Self(s))
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

#[cfg(test)]
mod tests {
    use crate::domain::{Address, Network};
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let address = "a".repeat(256);
        assert_ok!(Address:: parse(address));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let address = "a".repeat(257);
        assert_err!(Address::parse(address));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let address = " ".to_string();
        assert_err!(Address::parse(address));
    }

    #[test]
    fn empty_string_is_rejected() {
        let address = "".to_string();
        assert_err!(Address::parse(address));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let address = name.to_string();
            assert_err!(Address::parse(address));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let address = "Ursula Le Guin".to_string();
        assert_ok!(Address::parse(address));
    }

    #[test]
    fn a_valid_network_is_parsed_successfully() {
        let network = "ethereum".to_string();
        assert_eq!(Network::parse(network).unwrap().as_ref(), "eth");
        let network = "binance".to_string();
        assert_eq!(Network::parse(network).unwrap().as_ref(), "bsc");
    }

    #[test]
    fn an_unsupported_network_is_not_parsed() {
        let network = "superchain".to_string();
        assert_err!(Network::parse(network));
    }

}