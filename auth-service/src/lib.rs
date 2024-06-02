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

use std::error::Error;
use axum::{
    Router,
    serve::Serve,
    response::IntoResponse,
    http::{StatusCode}
};
use axum::routing::post;
use tower_http::services::ServeDir;

// The Application struct encapsulates application logic
pub struct Application {
    server: Serve<Router, Router>,
    // expose address as a public field, so it's accessible in tests
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        let app = Application {
            server,
            address
        };
        Ok(app)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on port {}", &self.address);
        self.server.await
    }
}

async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
