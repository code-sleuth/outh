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

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(email) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(value) => Ok(value.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::Secret;

    #[tokio::test]
    async fn test_add_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse(Secret::new("m@umbrella.corp".to_owned())).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        let result = store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await;
        assert!(result.is_ok());
        assert_eq!(store.codes.get(&email), Some(&(login_attempt_id, code)));
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse(Secret::new("m@umbrella.corp".to_string())).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        store
            .codes
            .insert(email.clone(), (login_attempt_id.clone(), code.clone()));

        let result = store.remove_code(&email).await;
        assert!(result.is_ok());
        assert_eq!(store.codes.get(&email), None);
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse(Secret::new("m@umbrella.corp".to_string())).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        store
            .codes
            .insert(email.clone(), (login_attempt_id.clone(), code.clone()));

        let result = store.get_code(&email).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (login_attempt_id, code));
    }

    #[tokio::test]
    async fn test_get_code_not_found() {
        let store = HashmapTwoFACodeStore::default();
        let email = Email::parse(Secret::new("m@umbrella.corp".to_string())).unwrap();

        let result = store.get_code(&email).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TwoFACodeStoreError::LoginAttemptIdNotFound
        );
    }
}
