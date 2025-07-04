use serde_json::Value;
use sword::prelude::*;

pub struct ExtensionsTestMiddleware;

impl Middleware for ExtensionsTestMiddleware {
    async fn handle(mut ctx: Context, next: Next) -> MiddlewareResult {
        ctx.extensions
            .insert::<String>("test_extension".to_string());

        Ok(next.run(ctx.into()).await)
    }
}

pub struct MwWithState;

impl Middleware for MwWithState {
    async fn handle(mut ctx: Context, next: Next) -> MiddlewareResult {
        let app_state = ctx.get_state::<Value>()?;

        ctx.extensions.insert::<u16>(8080);
        ctx.extensions.insert(app_state.clone());

        Ok(next.run(ctx.into()).await)
    }
}

pub struct RoleMiddleware;

impl MiddlewareWithConfig<Vec<&str>> for RoleMiddleware {
    async fn handle(roles: Vec<&str>, ctx: Context, next: Next) -> MiddlewareResult {
        dbg!(&roles);
        Ok(next.run(ctx.into()).await)
    }
}
