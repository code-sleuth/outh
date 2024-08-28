/*
   Copyright 2024 Ibrahim Mbaziira

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use validator::Validate;

#[derive(Debug, Clone, PartialEq, Hash, Eq, Validate)]
pub struct Email {
    #[validate(email)]
    pub email: String,
}

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {
        let candidate = Email { email: s.clone() };
        match candidate.validate() {
            Ok(_) => Ok(candidate),
            Err(e) => Err(format!("{} is not a valid email address. [{}]", s, e)),
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use rand;

    #[test]
    fn reject_empty_string() {
        let email = "".to_owned();
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn reject_email_missing_at_symbol() {
        let email = "u.org".to_owned();
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn reject_email_missing_subject() {
        let email = "@me.org".to_owned();
        assert!(Email::parse(email).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmail(pub String);

    impl quickcheck::Arbitrary for ValidEmail {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let mut rng = rand::thread_rng();
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn successfully_parse_valid_emails(valid_email: ValidEmail) -> bool {
        Email::parse(valid_email.0).is_ok()
    }
}
