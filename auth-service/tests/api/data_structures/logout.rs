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

use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;

use super::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "1passWordd",
        "require2FA": false,
    });

    let response = app.signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "1passWordd"
    });

    let response = app.login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("Missing auth token");

    assert!(!auth_cookie.value().is_empty());

    let token = auth_cookie.value();
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("Missing auth token");
    assert!(auth_cookie.value().is_empty());

    let banned_token_store = app.banned_token_store.read().await;
    let contains_token = banned_token_store
        .contains_token(token)
        .await
        .expect("failed to check if token is banned");
    assert!(contains_token);
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.logout().await;
    assert_eq!(
        response.status().as_u16(),
        400,
        "non 400 BAD REQUEST response"
    );

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);
    assert!(auth_cookie.is_none());

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("failed to deserialize response body into EErrorResponse")
            .error,
        "Missing auth token".to_owned()
    );
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "noTsoSecure",
        "require2FA": false
    });
    let response = app.signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "noTsoSecure",
    });
    let response = app.login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    assert!(!auth_cookie.value().is_empty());

    // 1st logout
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 200);
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    assert!(auth_cookie.value().is_empty());

    // 2nd logout
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("could not deserialize response body to ErrorResponse")
            .error,
        "Missing auth token".to_owned()
    );
}

#[tokio::test]
async fn ds_should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    // create invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("failed to parse url"),
    );
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 401);
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);
    assert!(auth_cookie.is_none());

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("could not deserialize response body to ErrorResponse")
            .error,
        "Invalid auth token".to_owned()
    );
}
