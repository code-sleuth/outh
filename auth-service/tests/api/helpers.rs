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

use auth_service::Application;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("failed to build service");

        let address = format!("http://{}", app.address.clone());

        // run auth service in a separate async task to avoid blocking of the main thread.
        #[allow(clippy::let_underscore_future)]
        tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        let svc = TestApp {
            address,
            http_client,
        };

        svc
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("failed to execute request")
    }

    pub async fn signup<SignupRequest>(&self, body: &SignupRequest) -> reqwest::Response
    where
    SignupRequest: serde::Serialize {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("signup failed")
    }

    pub async fn login<LoginRequest>(&self, body: &LoginRequest) -> reqwest::Response
    where
    LoginRequest: serde::Serialize {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("login failed")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("logout failed")
    }

    pub async fn verify_2fa<Request>(&self, body: &Request) -> reqwest::Response
    where
    Request: serde::Serialize {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("2fa verification failed")
    }

    pub async fn verify_token<Request>(&self, body: &Request) -> reqwest::Response
    where
    Request: serde::Serialize {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("token verification failed")
    }
}
