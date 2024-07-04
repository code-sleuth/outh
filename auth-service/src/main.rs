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

use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::{
    Application,
    app_state::AppState,
    services::hashmap_user_store::HashmapUserStore,
};

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:42069";
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store);
    let svc = Application::build(app_state, address)
        .await
        .expect("failed to build service");

    svc
        .run()
        .await
        .expect("failed to run service");
}
