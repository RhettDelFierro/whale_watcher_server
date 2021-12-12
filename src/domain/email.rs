use validator::validate_email;

#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid subscriber email.", s))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use claim::assert_err;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(Email:: parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "blahhahadomain.com".to_string();
        assert_err!(Email:: parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@somethingoranotherdomain.com".to_string();
        assert_err!(Email:: parse(email));
    }
}