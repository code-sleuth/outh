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

use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, Secret};
use std::hash::Hash;
use validator::ValidateEmail;

#[derive(Debug, Clone)]
pub struct Email(Secret<String>);

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

impl Eq for Email {}

impl Email {
    pub fn parse(s: Secret<String>) -> Result<Email> {
        if ValidateEmail::validate_email(s.expose_secret()) {
            Ok(Self(s))
        } else {
            Err(eyre!(format!(
                "{} is not a valid email.",
                s.expose_secret()
            )))
        }
    }
}

impl AsRef<Secret<String>> for Email {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use rand;
    use secrecy::Secret;

    #[test]
    fn reject_empty_string() {
        let email = Secret::new("".to_string());
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn reject_email_missing_at_symbol() {
        let email = Secret::new("u.org".to_string());
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn reject_email_missing_subject() {
        let email = Secret::new("@me.org".to_string());
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
        Email::parse(Secret::new(valid_email.0)).is_ok()
    }
}
