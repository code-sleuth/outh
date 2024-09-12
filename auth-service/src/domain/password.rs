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

#[derive(Debug, Clone)]
pub struct Password(Secret<String>);

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Password {
    pub fn parse(s: Secret<String>) -> Result<Password> {
        if validate_password(&s) {
            Ok(Self(s))
        } else {
            Err(eyre!("Failed to parse string to a Password type"))
        }
    }
}

fn validate_password(s: &Secret<String>) -> bool {
    s.expose_secret().len() >= 8
}

impl AsRef<Secret<String>> for Password {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Password;
    use fake::{faker::internet::en::Password as FakePassword, Fake};
    use rand;
    use secrecy::Secret;

    #[test]
    fn reject_empty_string() {
        let password = Secret::new("".to_string());
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn reject_string_less_than_8_characters() {
        let password = Secret::new("123456".to_owned());
        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub Secret<String>);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let mut rng = rand::thread_rng();
            let password = FakePassword(8..28).fake_with_rng(&mut rng);
            Self(Secret::new(password))
        }
    }

    #[quickcheck_macros::quickcheck]
    fn successfully_parse_valid_passwords(validate_password: ValidPasswordFixture) -> bool {
        Password::parse(validate_password.0).is_ok()
    }
}
