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
    routes::SignupResponse,
    ErrorResponse,
};

use crate::helpers::{
    get_random_email,
    TestApp,
};

#[tokio::test]
async fn signup() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let table_tests = [
        serde_json::json!({
            "password": "notSoSecure",
            "require2FA": true
        }),
        serde_json::json!({
            "require2FA": true,
            "email": random_email,
        }),
        serde_json::json!({
            "password": "notSoSecure",
            "email": random_email,
        }),
        serde_json::json!({
            "password": "notSoSecure",
            "email": random_email,
            "require2FA": "true"
        }),
        serde_json::json!({})
    ];

    for t in table_tests {
        let response = app.signup(&t).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            t
        );
    }
}
