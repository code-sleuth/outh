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

use auth_service::Application;
use auth_service::{
    app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType},
    domain::Email,
    services::data_stores::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore},
    services::postmark_email_client::PostmarkEmailClient,
    utils::constants::test,
};
use reqwest::{cookie::Jar, Client};
use secrecy::Secret;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use wiremock::MockServer;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub cookie_jar: Arc<Jar>,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
        let email_server = MockServer::start().await;
        let base_url = email_server.uri();
        let email_client = Arc::new(configure_postmark_email_client(base_url));
        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            email_client,
        );
        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("failed to build service");

        let address = format!("http://{}", app.address.clone());

        // run auth service in a separate async task to avoid blocking of the main thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
            http_client,
            cookie_jar,
            banned_token_store,
            two_fa_code_store,
            email_server,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("failed to execute request")
    }

    pub async fn signup<SignupRequest>(&self, body: &SignupRequest) -> reqwest::Response
    where
        SignupRequest: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("signup failed")
    }

    pub async fn login<LoginRequest>(&self, body: &LoginRequest) -> reqwest::Response
    where
        LoginRequest: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("login failed")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("logout failed")
    }

    pub async fn verify_2fa<Request>(&self, body: &Request) -> reqwest::Response
    where
        Request: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("2fa verification failed")
    }

    pub async fn verify_token<Request>(&self, body: &Request) -> reqwest::Response
    where
        Request: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("token verification failed")
    }
}

pub fn get_random_email() -> String {
    format!("{}@umbrella.corp", Uuid::new_v4())
}

fn configure_postmark_email_client(base_url: String) -> PostmarkEmailClient {
    let postmark_auth_token = Secret::new("auth_token".to_owned());

    let sender = Email::parse(Secret::new(test::email_client::SENDER.to_owned())).unwrap();

    let http_client = Client::builder()
        .timeout(test::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(base_url, sender, postmark_auth_token, http_client)
}
