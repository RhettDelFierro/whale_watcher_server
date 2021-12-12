use super::MAX_LIMIT_CHARACTERS;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct TokenName(String);

impl TokenName {
    pub fn parse(s: String) -> Result<TokenName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > MAX_LIMIT_CHARACTERS;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '_'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid TokenName.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for TokenName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::TokenName;
    use crate::domain::MAX_LIMIT_CHARACTERS;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let token_name = "a".repeat(MAX_LIMIT_CHARACTERS);
        assert_ok!(TokenName::parse(token_name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let token_name = "a".repeat(MAX_LIMIT_CHARACTERS + 1);
        assert_err!(TokenName::parse(token_name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let token_name = "".to_string();
        assert_err!(TokenName::parse(token_name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '_'] {
            let token_name = name.to_string();
            assert_err!(TokenName::parse(token_name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let token_name = "SomethingorAnother$".to_string();
        assert_ok!(TokenName::parse(token_name));
    }
}
