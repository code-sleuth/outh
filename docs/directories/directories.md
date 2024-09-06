# Directory structure of the project
```shell
├── LICENSE
├── Makefile
├── README.md
├── app-service
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── assets
│   │   ├── app.js
│   │   ├── default.jpg
│   │   └── logo.webp
│   ├── src
│   │   └── main.rs
│   └── templates
│       └── index.html
├── auth-service
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── api_schema.yml
│   ├── assets
│   │   ├── app.js
│   │   ├── index.html
│   │   └── logo.webp
│   ├── bin
│   │   └── lambda
│   │       └── lambda.rs
│   ├── build.rs
│   ├── lambda.arm.Dockerfile
│   ├── lambda.x86_64.Dockerfile
│   ├── migrations
│   │   ├── 20240904120306_create_users_table.down.sql
│   │   └── 20240904120306_create_users_table.up.sql
│   ├── samconfig.toml
│   ├── src
│   │   ├── app_state.rs
│   │   ├── domain
│   │   │   ├── data_stores.rs
│   │   │   ├── email.rs
│   │   │   ├── email_client.rs
│   │   │   ├── error.rs
│   │   │   ├── mod.rs
│   │   │   ├── password.rs
│   │   │   └── user.rs
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── routes
│   │   │   ├── login.rs
│   │   │   ├── logout.rs
│   │   │   ├── mod.rs
│   │   │   ├── signup.rs
│   │   │   ├── verify_2fa.rs
│   │   │   └── verify_token.rs
│   │   ├── services
│   │   │   ├── data_stores
│   │   │   │   ├── hashmap_two_fa_code_store.rs
│   │   │   │   ├── hashmap_user_store.rs
│   │   │   │   ├── hashset_banned_token_store.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── postgres_user_store.rs
│   │   │   │   ├── redis_banned_token_store.rs
│   │   │   │   └── redis_two_fa_code_store.rs
│   │   │   ├── mock_email_client.rs
│   │   │   └── mod.rs
│   │   └── utils
│   │       ├── auth.rs
│   │       ├── constants.rs
│   │       └── mod.rs
│   ├── template.yaml
│   └── tests
│       └── api
│           ├── data_structures
│           │   ├── helpers.rs
│           │   ├── login.rs
│           │   ├── logout.rs
│           │   ├── mod.rs
│           │   ├── root.rs
│           │   ├── signup.rs
│           │   ├── verify_2fa.rs
│           │   └── verify_token.rs
│           ├── main.rs
│           └── postgres
│               ├── helpers.rs
│               ├── login.rs
│               ├── logout.rs
│               ├── mod.rs
│               ├── root.rs
│               ├── signup.rs
│               ├── verify_2fa.rs
│               └── verify_token.rs
├── compose.override.yml
├── compose.yml
├── docs
│   ├── architecture
│   ├── directories
│   │   └── directories.md
│   ├── license
│   │   └── header.txt
│   └── releases
│       └── v0.0.0.md
├── m.txt
└── scripts
    └── license.sh
```
