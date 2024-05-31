# OUTH Service

## Setup & Build
```shell
make build
```

## Run services locally
#### App service
```shell
make run-app-service
```

checkout http://localhost:42068

#### Auth service
```shell
make run-auth-service
```

checkout http://localhost:42069

## Run services locally (Docker)
```shell
make docker
```

checkout http://localhost:42068 and http://localhost:42069

## Test services
#### App service
```shell
make test-app-service
```

#### Auth service
```shell
make test-auth-service
```
