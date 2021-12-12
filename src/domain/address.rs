use unicode_segmentation::UnicodeSegmentation;
use super::MAX_LIMIT_CHARACTERS;

#[derive(Debug)]
pub struct Address(String);

impl Address {
    pub fn parse(s: String) -> Result<Address, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > MAX_LIMIT_CHARACTERS;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid address.", s))
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

#[cfg(test)]
mod tests {
    use crate::domain::MAX_LIMIT_CHARACTERS;
    use super::Address;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let address = "a".repeat(MAX_LIMIT_CHARACTERS);
        assert_ok!(Address:: parse(address));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let address = "a".repeat(MAX_LIMIT_CHARACTERS + 1);
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
        let address = "Something or Another".to_string();
        assert_ok!(Address::parse(address));
    }
}