#    Copyright 2024 Ibrahim Mbaziira
#
#    Licensed under the Apache License, Version 2.0 (the "License");
#    you may not use this file except in compliance with the License.
#    You may obtain a copy of the License at
#
#        http://www.apache.org/licenses/LICENSE-2.0
#
#    Unless required by applicable law or agreed to in writing, software
#    distributed under the License is distributed on an "AS IS" BASIS,
#    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#    See the License for the specific language governing permissions and
#    limitations under the License.
#

# Start with image that has the Rust toolchain installed
FROM rust:1.77-alpine AS chef
USER root
# Add cargo-chef to cache dependencies
RUN apk add --no-cache musl-dev & cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
# Capture info needed to build dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --bin auth-service --recipe-path recipe.json

# Build application
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin lambda_binary

FROM public.ecr.aws/lambda/provided:al2-x86_64
WORKDIR /app
ENV AWS_LAMBDA_FUNCTION_NAME="auth-service"
ENV JWT_SECRET="notSoSecret"
ENV REDIS_HOST_NAME=redis
COPY --from=builder /app/target/release/lambda_binary ${LAMBDA_RUNTIME_DIR}/bootstrap
COPY --from=builder /app/assets /app/assets
CMD ["bootstrap"]
