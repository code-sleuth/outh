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
    services::{
        hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore,
    },
    utils::constants::prod,
    Application,
};
use http::Request as HttpRequest;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceExt;

use axum::http::{Method, Uri};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // init app state
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let app_state = AppState::new(user_store, banned_token_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("failed to build service");

    let response = handle_lambda_event(app, event).await?;

    Ok(response)
}

async fn handle_lambda_event(app: Application, event: Request) -> Result<Response<Body>, Error> {
    // convert Lambda request to Axum request
    let (parts, body) = event.into_parts();
    let uri = Uri::from_str(&parts.uri.path()).unwrap();
    let method = Method::from_str(parts.method.as_str()).unwrap();

    let http_body = match body {
        Body::Empty => axum::body::Body::empty(),
        Body::Text(text) => axum::body::Body::from(text),
        Body::Binary(data) => axum::body::Body::from(data),
    };

    let http_request = HttpRequest::builder()
        .uri(uri)
        .method(method)
        .body(http_body)
        .unwrap();

    // process request using the Router
    let axum_response = app.router.oneshot(http_request).await.unwrap();

    // convert Axum response to Lambda response
    let (parts, body) = axum_response.into_parts();
    let lambda_body = match axum::body::to_bytes(body, 100000000).await {
        Ok(bytes) => Body::Binary(bytes.to_vec()),
        Err(_) => Body::Empty,
    };

    let mut lambda_response = Response::builder()
        .status(parts.status)
        .body(lambda_body)
        .unwrap();

    // handle (copy) headers
    for (key, value) in parts.headers.iter() {
        lambda_response.headers_mut().insert(key, value.clone());
    }

    Ok(lambda_response)
}
