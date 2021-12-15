use super::MAX_LIMIT_CHARACTERS;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Notes(String);

impl Notes {
    pub fn parse(s: Option<String>) -> Result<Notes, String> {
        match s {
            Some(s) => {
                let is_too_long = s.graphemes(true).count() > MAX_LIMIT_CHARACTERS;
                let forbidden_characters = ['"', '<', '>', '\\', '{', '}', '_'];
                let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
                if is_too_long || contains_forbidden_characters {
                    Err(format!("{} is not a valid token name.", s))
                } else {
                    Ok(Self(s))
                }
            },
            None => {
                Ok(Self("".to_owned()))
            }
        }

    }
}

impl AsRef<str> for Notes {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Notes;
    use crate::domain::MAX_LIMIT_CHARACTERS;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let notes = "a".repeat(MAX_LIMIT_CHARACTERS);
        assert_ok!(Notes::parse(notes));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let notes = "a".repeat(MAX_LIMIT_CHARACTERS + 1);
        assert_err!(Notes::parse(notes));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for note in &['"', '<', '>', '\\', '{', '}', '_'] {
            let char = note.to_string();
            assert_err!(Notes::parse(char));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let notes = "SomethingorAnother$".to_string();
        assert_ok!(Notes::parse(notes));
    }
}
