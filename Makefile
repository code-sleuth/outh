#  Copyright 2024 Ibrahim Mbaziira
#
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.

APP_SERVICE=app-service
AUTH_SERVICE=auth-service
COMMIT=$(shell sh -c 'git rev-parse --short HEAD')

.PHONY: build run docker

build: build-app-service build-auth-service

run: run-app-service run-auth-service

docker: docker-build docker-run

build-app-service:
	cargo install cargo-watch && cd $(shell pwd)/app-service && cargo build

build-auth-service:
	cargo install cargo-watch && cd $(shell pwd)/auth-service && cargo build

run-app-service:
	cd $(shell pwd)/app-service && cargo watch -q -c -w src/ -w assets/ -w templates/ -x run

run-auth-service:
	cd $(shell pwd)/auth-service && cargo watch -q -c -w src/ -w assets/ -x run

test-app-service:
	cd $(shell pwd)/app-service && cargo test

test-auth-service:
	cd $(shell pwd)/auth-service && cargo test

docker-build:
	docker compose build

docker-run:
	docker compose up
