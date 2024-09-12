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
use auth_service::{
    domain::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME, ErrorResponse,
};
use secrecy::{ExposeSecret, Secret};
use test_helpers::api_test;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

#[api_test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    //let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "notSoSecure1",
        "require2FA": false,
    });
    let response = app.signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "notSoSecure1",
    });
    let response = app.login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    assert!(!auth_cookie.value().is_empty());
}

#[api_test]
async fn should_return_400_if_invalid_input() {
    //let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "notSoSecure1",
        "require2FA": false
    });

    let response = app.signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let test_cases = vec![
        ("invalid_email", "password123"),
        (random_email.as_str(), "invalid"),
        ("", "password123"),
        (random_email.as_str(), ""),
        ("", ""),
    ];

    for (email, password) in test_cases {
        let login_body = serde_json::json!({
            "email": email,
            "password": password
        });
        let response = app.login(&login_body).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            login_body
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[api_test]
async fn should_return_401_if_incorrect_credentials() {
    //let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "notSoSecret",
        "require2FA": false
    });

    let response = app.signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let test_cases = vec![
        (random_email.as_str(), "inCoRrecT"),
        ("ap@umbrella.corp", "zoMbiEEs"),
        ("x@0xfrait.com", "+#$^^<>?@@%"),
    ];

    for (email, password) in test_cases {
        let login_body = serde_json::json!({
            "email": email,
            "password": password
        });

        let response = app.login(&login_body).await;

        assert_eq!(
            response.status().as_u16(),
            401,
            "failed for case: {:?}",
            login_body
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("failed to deserialize response into error")
                .error,
            "Incorrect credentials".to_owned()
        );
    }
}

#[api_test]
async fn should_return_422_if_malformed_credentials() {
    //let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "noTsoSecret",
        "require2FA": false
    });

    let response = app.signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let test_cases = [
        serde_json::json!({
            "password": "simplepass2"
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "failed for case: {:?}",
            test_case
        );
    }
}
