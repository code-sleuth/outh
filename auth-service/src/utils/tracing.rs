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

use color_eyre::eyre::Result;
use std::time::Duration;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use axum::{body::Body, extract::Request, response::Response};
use tracing::{Level, Span};

pub fn init_tracing() -> Result<()> {
    let fmt_layer = fmt::layer().compact();

    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}

pub fn make_span_with_request_id(request: &Request<Body>) -> Span {
    let request_id = uuid::Uuid::new_v4();
    tracing::span!(
        Level::INFO,
        "[REQUEST]",
        method = tracing::field::display(request.method()),
        uri = tracing::field::display(request.uri()),
        version = tracing::field::debug(request.version()),
        request_id = tracing::field::display(request_id),
    )
}

pub fn on_request(_request: &Request<Body>, _span: &Span) {
    tracing::event!(Level::INFO, "[REQUEST START]");
}

pub fn on_response(response: &Response, latency: Duration, _span: &Span) {
    let status = response.status();
    let status_code = status.as_u16();
    let status_code_class = status_code / 100;

    match status_code_class {
        4..=5 => {
            tracing::event!(
                Level::ERROR,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }

        _ => {
            tracing::event!(
                Level::INFO,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }
    }
}
