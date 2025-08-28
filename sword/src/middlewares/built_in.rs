use axum::middleware::Next;
use axum_responses::http::HttpResponse;

use crate::{
    next,
    web::{Context, MiddlewareResult},
};

const NO_BODY_METHODS: [&str; 4] = ["GET", "DELETE", "HEAD", "OPTIONS"];

pub async fn content_type_check(ctx: Context, next: Next) -> MiddlewareResult {
    let method = ctx.method().as_str();
    let content_type = ctx.header("Content-Type").unwrap_or_default();

    if NO_BODY_METHODS.contains(&method) {
        return next!(ctx, next);
    }

    if content_type != "application/json" && !content_type.contains("multipart/form-data") {
        return Err(HttpResponse::UnsupportedMediaType().message(
            "Only application/json and multipart/form-data content types are supported.",
        ));
    }

    next!(ctx, next)
}
