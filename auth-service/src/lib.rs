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

use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use domain::AuthAPIError;
use redis::{Client, RedisResult};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::error::Error;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use utils::tracing::{make_span_with_request_id, on_request, on_response};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

use crate::app_state::AppState;
use routes::{login, logout, signup, verify_2fa, verify_token};

// The Application struct encapsulates application logic
pub struct Application {
    pub server: Serve<Router, Router>,
    // expose address as a public field, so it's accessible in tests
    pub address: String,
    pub router: Router,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "https://localhost:42069".parse()?,
            "https://auth.0xfrait.com".parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router.clone());

        let app = Application {
            server,
            address,
            router,
        };
        Ok(app)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        log_error_chain(&self);

        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED, "Incorrect credentials")
            }
            AuthAPIError::UnexpectedError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}

pub async fn get_postgres_pool(url: &Secret<String>) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(url.expose_secret())
        .await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}

fn log_error_chain(e: &(dyn Error + 'static)) {
    let separator =
        "\n----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}
