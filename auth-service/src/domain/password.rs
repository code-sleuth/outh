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

use validator::{
    Validate
};
use serde::Deserialize;

#[derive(Debug, Eq, Hash, Clone, PartialEq, Validate, Deserialize)]
pub struct Password{
    #[validate(length(min = 8))]
    pub password: String
}

impl Password {
    pub fn parse(s: String) -> Result<Password, String> {
        let candidate = Password { password: s.clone() };
        match candidate.validate() {
            Ok(_) => Ok(candidate),
            Err(e) => {
                Err(format!("{} password validation failed. [{}]", s, e))
            }
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.password
    }
}

#[cfg(test)]
mod tests {
    use super::Password;
    use fake::{
        Fake,
        faker::internet::en::Password as FakePassword
    };
    use rand;

    #[test]
    fn reject_empty_string() {
        let password = "".to_owned();
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn reject_string_less_than_8_characters() {
        let password = "123456".to_owned();
        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct  ValidatePassword(pub String);

    impl quickcheck::Arbitrary for ValidatePassword {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let mut rng = rand::thread_rng();
            let password = FakePassword(8..28).fake_with_rng(&mut rng);
            Self(password)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn successfully_parse_valid_passwords(validate_password: ValidatePassword) -> bool {
        Password::parse(validate_password.0).is_ok()
    }
}