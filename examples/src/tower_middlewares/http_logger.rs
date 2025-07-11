use std::time::Duration;

use axum::http;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{MakeSpan, OnRequest, OnResponse, TraceLayer};
use tracing::Span;

#[derive(Clone, Debug)]
pub struct HttpLogger {
    pub layer: TraceLayer<
        SharedClassifier<ServerErrorsAsFailures>,
        TraceMakeSpan,
        TraceOnRequest,
        TraceOnResponse,
    >,
}

impl HttpLogger {
    pub fn new() -> Self {
        tracing_subscriber::fmt()
            .with_target(false)
            .compact()
            .init();

        HttpLogger {
            layer: TraceLayer::new_for_http()
                .make_span_with(TraceMakeSpan)
                .on_request(TraceOnRequest)
                .on_response(TraceOnResponse),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TraceMakeSpan;

impl<B> MakeSpan<B> for TraceMakeSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> Span {
        let method = request.method().as_str();
        let path = request.uri().path();

        tracing::info_span!("request", %method, %path)
    }
}

#[derive(Clone, Debug)]
pub struct TraceOnRequest;

impl<B> OnRequest<B> for TraceOnRequest {
    fn on_request(&mut self, request: &http::Request<B>, _: &Span) {
        tracing::info!("HTTP - [{}] - [{}]", request.method(), request.uri().path());
    }
}

#[derive(Clone, Debug)]
pub struct TraceOnResponse;

impl<B> OnResponse<B> for TraceOnResponse {
    fn on_response(self, response: &http::Response<B>, latency: Duration, _: &Span) {
        tracing::info!(
            "HTTP - [{}] - [{}ms]",
            response.status().as_u16(),
            latency.as_millis()
        );
        println!();
    }
}
