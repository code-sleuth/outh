# Directory structure of the project
```shell
.
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── api_schema.yml
├── assets
│   ├── app.js
│   ├── index.html
│   └── logo.webp
├── bin
│   └── lambda
│       └── lambda.rs
├── lambda.arm.Dockerfile
├── lambda.x86_64.Dockerfile
├── m.txt
├── samconfig.toml
├── src
│   ├── app_state.rs
│   ├── domain
│   │   ├── data_stores.rs
│   │   ├── email.rs
│   │   ├── error.rs
│   │   ├── mod.rs
│   │   ├── password.rs
│   │   └── user.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── routes
│   │   ├── login.rs
│   │   ├── logout.rs
│   │   ├── mod.rs
│   │   ├── signup.rs
│   │   ├── verify_2fa.rs
│   │   └── verify_token.rs
│   ├── services
│   │   ├── hashmap_user_store.rs
│   │   ├── hashset_banned_token_store.rs
│   │   └── mod.rs
│   └── utils
│       ├── auth.rs
│       ├── constants.rs
│       └── mod.rs
├── template.yaml
└── tests
    └── api
        ├── helpers.rs
        ├── login.rs
        ├── logout.rs
        ├── main.rs
        ├── root.rs
        ├── signup.rs
        ├── verify_2fa.rs
        └── verify_token.rs
```
