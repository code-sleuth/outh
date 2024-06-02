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

use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui(){
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html")
}

#[tokio::test]
async fn signup(){
    let app = TestApp::new().await;
    let body = vec![("username", "john"), ("password", "notSoSecurePassword")];
    let response = app.signup(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login(){
    let app = TestApp::new().await;
    let body = vec![("username", "john"), ("password", "notSoSecurePassword")];
    let response = app.login(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout(){
    let app = TestApp::new().await;
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa(){
    let app = TestApp::new().await;
    let body = vec![("username", "john"), ("password", "notSoSecurePassword")];
    let response = app.verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token(){
    let app = TestApp::new().await;
    let body = vec![("token", "jwt")];
    let response = app.verify_token(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}
