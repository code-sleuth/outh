# OUTH Service

## Environment
Its a prerequisite that these environment variables are set. Set them in your terminal.

```bash
$ export JWT_SECRET=<your-jwt-secret>
$ export DATABASE_URL=<example-postgres://postgres:notSoSecret@postgres:5432>
$ export POSTMARK_AUTH_TOKEN=<your-postmark-auth-token>
```


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
