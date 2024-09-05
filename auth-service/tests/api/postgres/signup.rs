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
use auth_service::{routes::SignupResponse, ErrorResponse};
use test_helpers::api_test;

#[api_test]
async fn should_return_201_if_valid_input() {
    //let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "notSoSecure",
        "require2FA": true
    });
    let response = app.signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[api_test]
async fn should_return_400_if_invalid_input() {
    //let app = TestApp::new().await;

    let random_email = get_random_email();

    let input = [
        serde_json::json!({
            "email": "",
            "password": "notSoSecure",
            "require2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "",
            "require2FA": true
        }),
        serde_json::json!({
            "email": "",
            "password": "",
            "require2FA": true
        }),
        serde_json::json!({
            "email": "invalid_email",
            "password": "password123",
            "require2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "invalid",
            "require2FA": true
        }),
    ];
    for i in input.iter() {
        let response = app.signup(i).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);
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
async fn should_return_409_if_email_already_exists() {
    //let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "notSoSecure",
        "require2FA": true
    });
    let response = app.signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    let response = app.signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 409);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}

#[api_test]
async fn should_return_422_if_malformed_input() {
    //let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "notSoSecure",
            "require2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "require2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "notSoSecure",
        }),
        serde_json::json!({
            "email": random_email,
            "password": "notSoSecure",
            "require2FA": "true"
        }),
        serde_json::json!({}),
    ];
    for test_case in test_cases.iter() {
        let response = app.signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
