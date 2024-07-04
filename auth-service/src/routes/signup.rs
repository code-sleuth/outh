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
    http::StatusCode,
    Json,
    response::IntoResponse,
    extract::State,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    app_state::AppState,
    domain::{
        AuthAPIError,
        Email,
        Password,
        User,
    }
};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "require2FA")]
    pub require_2fa: bool
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = User::new(email, password, request.require_2fa);
    let mut user_store = state.user_store.write().await;

    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    if user_store.add_user(user).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_owned(),
    });

    Ok((StatusCode::CREATED, response))
}
