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
│   ├── src
│   │   ├── lib.rs
│   │   └── main.rs
│   └── tests
│       └── api
│           ├── helpers.rs
│           ├── main.rs
│           └── routes.rs
├── compose.override.yml
├── compose.yml
└── docs
    ├── architecture
    ├── directories
    │   └── directories.md
    └── releases
        └── v0.0.0.md
```