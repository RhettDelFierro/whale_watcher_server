use super::MAX_LIMIT_CHARACTERS;
use sqlx::Type;
use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Clone, sqlx::Type)]
pub enum AddressType {
    Exchange,
    LiquidityLocker,
    TokenCreator,
    Whale,
    Legit,
    LongTermHolder,
    MediumTermHolder,
    ShortTermHolder,
    SuspiciousHolder,
    Scammer,
    Paperhand,
    Dumper,
    PanicSeller,
    DeadAddress,
}

// can probably have this return an enum for address_type.
fn derive_address_type(s: &str) -> Result<AddressType, String> {
    let str = s.to_lowercase();
    if str == "exchange" {
        return Ok(AddressType::Exchange);
    } else if str == "liquiditylocker" || str == "liquidity_locker" {
        return Ok(AddressType::LiquidityLocker);
    } else if str == "creator" || str == "token_creator" || str == "tokencreator" {
        return Ok(AddressType::TokenCreator);
    } else if str == "whale" {
        return Ok(AddressType::Whale);
    } else if str == "legit" || str == "legit_address" || str == "legitaddress" {
        return Ok(AddressType::Legit);
    } else if str == "longtermholder" || str == "long_term_holder" || str == "longterm_holder" {
        return Ok(AddressType::LongTermHolder);
    } else if str == "mediumtermholder" || str == "medium_term_holder" || str == "mediumterm_holder"
    {
        return Ok(AddressType::MediumTermHolder);
    } else if str == "shorttermholder" || str == "short_term_holder" || str == "shortterm_holder" {
        return Ok(AddressType::ShortTermHolder);
    } else if str == "suspiciousholder" || str == "suspicious_holder" || str == "suspicious" {
        return Ok(AddressType::SuspiciousHolder);
    } else if str == "scammer" || str == "scam" {
        return Ok(AddressType::Scammer);
    } else if str == "paper_hand" || str == "paperhand" {
        return Ok(AddressType::Paperhand);
    } else if str == "dumper" {
        return Ok(AddressType::Dumper);
    } else if str == "panic_seller" || str == "panicseller" {
        return Ok(AddressType::PanicSeller);
    } else if str == "dead_address" || str == "dead" || str == "deadaddress" {
        return Ok(AddressType::DeadAddress);
    }
    Err(format!("{} address type not supported", s))
}

impl AddressType {
    pub fn parse(s: String) -> Result<AddressType, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > MAX_LIMIT_CHARACTERS;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid address type.", s))
        } else {
            derive_address_type(&s)
        }
    }
}

impl AsRef<str> for AddressType {
    fn as_ref(&self) -> &str {
        match self {
            AddressType::ETH => "eth",
            AddressType::BSC => "bsc",
            AddressType::ADA => "ada",
            AddressType::AVAX => "avax",
            AddressType::MATIC => "matic",
            AddressType::FTM => "ftm",
            AddressType::SOL => "sol",
            AddressType::LUNA => "luna",
            AddressType::DOT => "dot",
            AddressType::MOVR => "movr",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AddressType;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_valid_address_type_is_parsed_successfully() {
        let address_type = "ethereum".to_string();
        assert_eq!(AddressType::parse(address_type).unwrap().as_ref(), "eth");
        let address_type = "binance".to_string();
        assert_eq!(AddressType::parse(address_type).unwrap().as_ref(), "bsc");
    }

    #[test]
    fn an_unsupported_address_type_is_not_parsed() {
        let address_type = "somesuperchain".to_string();
        assert_err!(AddressType::parse(address_type));
    }
}
