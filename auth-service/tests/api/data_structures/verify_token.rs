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

use super::helpers::{get_random_email, TestApp};
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};

#[tokio::test]
async fn should_return_200_for_valid_token() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "noTsoSecure",
        "require2FA": false,
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

    let token = auth_cookie.value();
    let verify_token_body = serde_json::json!({
        "token": token,
    });
    let response = app.verify_token(&verify_token_body).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    let test_cases = vec!["", "invalid_token"];
    for test_case in test_cases {
        let verify_token_body = serde_json::json!({
            "token": test_case
        });
        let response = app.verify_token(&verify_token_body).await;
        assert_eq!(response.status().as_u16(), 401);
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("could not deserialize response body to ErrorResponse")
                .error,
            "Invalid auth token".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "notSoSecure",
        "require2FA": false
    });
    let response = app.signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "notSoSecure",
    });
    let response = app.login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    assert!(!auth_cookie.value().is_empty());

    let token = auth_cookie.value();
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let verify_token_body = serde_json::json!({
        "token": token,
    });
    let response = app.verify_token(&verify_token_body).await;
    assert_eq!(response.status().as_u16(), 401);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("could not deserialize response body to ErrorResponse")
            .error,
        "Invalid auth token".to_owned()
    );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let test_cases = vec![
        serde_json::json!({
            "token": true,
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.verify_token(&test_case).await;
        assert_eq!(response.status().as_u16(), 422);
    }
}
