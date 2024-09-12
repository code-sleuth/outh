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

use auth_service::{
    app_state::AppState,
    get_postgres_pool, get_redis_client,
    domain::Email,
    services::{
        data_stores::{
            // HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore,
            PostgresUserStore,
            RedisBannedTokenStore,
            RedisTwoFACodeStore,
        },
        //mock_email_client::MockEmailClient,
        postmark_email_client::PostmarkEmailClient,
    },
    utils::{
        constants::{prod, DATABASE_URL, POSTMARK_AUTH_TOKEN, REDIS_HOST_NAME},
        tracing::init_tracing,
    },
    Application,
};
use reqwest::Client;
use secrecy::Secret;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing().expect("Failed to initialize tracing");

    let pg_pool = configure_postgresql().await;
    let redis_connection = Arc::new(RwLock::new(configure_redis()));

    // use data structures
    // let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    // let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    // let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));

    // use persistent storage
    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_connection.clone(),
    )));
    let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));

    let email_client = Arc::new(configure_postmark_email_client());
    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );
    let svc = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("failed to build service");

    svc.run().await.expect("failed to run service");
}

async fn configure_postgresql() -> PgPool {
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}

fn configure_postmark_email_client() -> PostmarkEmailClient {
    let http_client = Client::builder()
        .timeout(prod::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(
        prod::email_client::BASE_URL.to_owned(),
        Email::parse(Secret::new(prod::email_client::SENDER.to_owned())).unwrap(),
        POSTMARK_AUTH_TOKEN.to_owned(),
        http_client,
    )
}
