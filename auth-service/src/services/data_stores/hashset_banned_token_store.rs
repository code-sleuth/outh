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

use secrecy::{ExposeSecret, Secret};
use std::collections::HashSet;

use crate::domain::data_stores::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError> {
        self.tokens.insert(token.expose_secret().to_owned());
        Ok(())
    }

    async fn contains_token(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError> {
        Ok(self.tokens.contains(token.expose_secret()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = Secret::new("the_token".to_owned());
        let result = store.add_token(token.clone()).await;

        assert!(result.is_ok());
        assert!(store.tokens.contains(token.expose_secret()))
    }

    #[tokio::test]
    async fn test_contains_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = Secret::new("the_token".to_owned());
        store.tokens.insert(token.expose_secret().to_owned());
        let result = store.contains_token(&token).await;
        assert!(result.unwrap());
    }
}
