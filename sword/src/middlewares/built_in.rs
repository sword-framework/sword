use crate::middlewares::Next;
use crate::web::HttpResponse;

use crate::{
    next,
    web::{Context, MiddlewareResult},
};

pub struct ContentTypeCheck;

impl ContentTypeCheck {
    pub async fn layer(ctx: Context, next: Next) -> MiddlewareResult {
        let content_type = ctx.header("Content-Type").unwrap_or_default();

        if !ctx.has_body() {
            return next!(ctx, next);
        }

        if content_type != "application/json" && !content_type.contains("multipart/form-data") {
            return Err(HttpResponse::UnsupportedMediaType().message(
                "Only application/json and multipart/form-data content types are supported.",
            ));
        }

        next!(ctx, next)
    }
}
