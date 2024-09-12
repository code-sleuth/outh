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

use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::Secret;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: Email::parse(Secret::new("ibrahim@umbrella.corp".to_owned())).unwrap(),
            password: Password::parse(Secret::new("$3curedZ".to_owned())).unwrap(),
            require_2fa: false,
        };

        // add a new user
        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        // test duplicate entries
        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let email = Email::parse(Secret::new("ibrahim@umbrella.corp".to_owned())).unwrap();

        let user = User {
            email: email.clone(),
            password: Password::parse(Secret::new("$3curedZ".to_owned())).unwrap(),
            require_2fa: false,
        };

        // get existing user
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.get_user(&email).await;
        assert_eq!(result, Ok(user));

        // get non existing user
        let result = user_store
            .get_user(&Email::parse(Secret::new("i@umbrella.corp".to_owned())).unwrap())
            .await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let email = Email::parse(Secret::new("ibrahim@umbrella.corp".to_owned())).unwrap();
        let password = Password::parse(Secret::new("$3curedZ".to_owned())).unwrap();

        let user = User {
            email: email.clone(),
            password: password.clone(),
            require_2fa: false,
        };

        // validate a user that exists with correct password
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        //  validate a user that exists with incorrect password
        let wrong_password = Password::parse(Secret::new("incorrectPassword".to_owned())).unwrap();
        let result = user_store.validate_user(&email, &wrong_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        //  validate a user that doesn't exist
        let result = user_store
            .validate_user(
                &Email::parse(Secret::new("i@umbrella.corp".to_string())).unwrap(),
                &password,
            )
            .await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
