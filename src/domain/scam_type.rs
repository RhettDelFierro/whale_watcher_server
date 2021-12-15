use super::MAX_LIMIT_CHARACTERS;
use sqlx::Type;
use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Clone, sqlx::Type)]
pub enum ScamType {
    HoneyPot,
    LiquidityPull,
    RugPull,
}

fn derive_scam(s: &str) -> Result<ScamType, String> {
    let str = s.to_lowercase();
    if str == "rug_pull" || str == "rug pull" || str == "rugpull" {
        return Ok(ScamType::RugPull);
    } else if str == "liquidity_pull" || str == "liquiditypull" || str == "liquidity pull" {
        return Ok(ScamType::LiquidityPull);
    } else if str == "honeypot" || str == "honey pot" || str == "honey_pot" {
        return Ok(ScamType::HoneyPot);
    }
    Err(format!(
        "{} scam type not supported: please use either rugpull, liquiditypull or honeypot",
        s
    ))
}

impl ScamType {
    pub fn parse(s: String) -> Result<ScamType, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > MAX_LIMIT_CHARACTERS;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid scam format.", s))
        } else {
            derive_scam(&s)
        }
    }
}

impl AsRef<str> for ScamType {
    fn as_ref(&self) -> &str {
        match self {
            ScamType::LiquidityPull => "liquidity_pull",
            ScamType::RugPull => "rug_pull",
            ScamType::HoneyPot => "honeypot",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ScamType;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_valid_scam_type_is_parsed_successfully() {
        let scam_type = "rugpull".to_string();
        assert_eq!(ScamType::parse(scam_type).unwrap().as_ref(), "rug_pull");
        let scam_type = "rug_pull".to_string();
        assert_eq!(ScamType::parse(scam_type).unwrap().as_ref(), "rug_pull");
        let scam_type = "RuG pUlL".to_string();
        assert_eq!(ScamType::parse(scam_type).unwrap().as_ref(), "rug_pull");
    }

    #[test]
    fn an_unsupported_network_is_not_parsed() {
        let scam_type = "some fake scam".to_string();
        assert_err!(ScamType::parse(scam_type));
    }
}
