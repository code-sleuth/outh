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

FROM public.ecr.aws/lambda/provided:al2-arm64 as builder

RUN yum install -y gcc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/app
COPY . .

RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo build --release --target aarch64-unknown-linux-gnu --bin lambda_binary

FROM public.ecr.aws/lambda/provided:al2-arm64
ENV AWS_LAMBDA_FUNCTION_NAME="auth-service"
ENV JWT_SECRET="notSoSecret"
COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-gnu/release/lambda_binary ${LAMBDA_RUNTIME_DIR}/bootstrap
CMD ["bootstrap"]