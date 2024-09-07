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
    domain::Email,
    services::{
        data_stores::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore},
        //mock_email_client::MockEmailClient,
        postmark_email_client::PostmarkEmailClient,
    },
    utils::constants::{prod, POSTMARK_AUTH_TOKEN},
    Application,
};
use axum::Router;
use http::Request as HttpRequest;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use reqwest::Client;
use secrecy::Secret;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // init app state
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(configure_postmark_email_client());
    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("failed to build service");
    let router = app.router;

    run(service_fn(move |event: Request| {
        let router = router.clone();
        async move { handle_lambda_event(router, event).await }
    }))
    .await
}

async fn handle_lambda_event(router: Router, event: Request) -> Result<Response<Body>, Error> {
    // convert Lambda request to Axum request
    let (parts, body) = event.into_parts();

    let http_body = match body {
        Body::Empty => axum::body::Body::empty(),
        Body::Text(text) => axum::body::Body::from(text),
        Body::Binary(data) => axum::body::Body::from(data),
    };

    let http_request = HttpRequest::from_parts(parts, http_body);

    // process request using the Router
    let axum_response = router.oneshot(http_request).await?;

    // convert Axum response to Lambda response
    let (parts, body) = axum_response.into_parts();
    let body_bytes = axum::body::to_bytes(body, 100000000).await?;
    let lambda_body = if body_bytes.is_empty() {
        Body::Empty
    } else {
        Body::Binary(body_bytes.to_vec())
    };

    let mut builder = Response::builder().status(parts.status);
    if let Some(headers) = builder.headers_mut() {
        for (key, value) in parts.headers.iter() {
            headers.append(key, value.clone());
        }
    }

    Ok(builder.body(lambda_body)?)
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
