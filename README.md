# OUTH Service

## Setup & Build
```bash
cargo install cargo-watch
cd app-service
cargo build
cd ..
cd auth-service
cargo build
cd ..
```

## Run servers locally
#### App service
```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

checkout http://localhost:42068

#### Auth service
```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

checkout http://localhost:42069

## Run servers locally (Docker)
```bash
docker compose build
docker compose up
```

checkout http://localhost:42068 and http://localhost:42069
