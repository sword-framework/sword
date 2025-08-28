<div align="center">
<img src="https://pillan.inf.uct.cl/~lrevillod/images/sword-logo.png" alt="Sword Logo" width="200">

<h1>⚔️ Sword ⚔️</h1>
<p><em>Rust web framework</em></p>
</div>

## ✨ Features

- 🛣️ **Macro-based routing** - Clean and intuitive route definitions
- 📄 **JSON-first design** - Built with JSON formats as priority
- ✅ **Built-in validation** - Support with `serde` and `validator` crates
- 🌐 **RFC-compliant HTTP responses** - Using `axum_responses` crate
- � **Express-Like** - It provides a `Context` object with utility methods for request handling
- �💉 **Dependency Injection** - Built-in DI support using `shaku` crate
- 🧩 **Middleware support** - Easily add middleware to routes or controllers
- 🚀 **Asynchronous by default** - Built on top of `axum` and `tokio`

## 🛠️ Usage

### Add to your `Cargo.toml`

```toml
[dependencies]
sword = "0.1.7"
tokio = { version = "^1", features = ["full"] }

# validation features:
validator = { version = "0.20.0", features = ["derive"] }

# JSON handling features:
serde = { version = "*", features = ["derive"] }
serde_json = "*"

# OPTIONAL: If you want to use dependency injection features
shaku = { version = "0.6.2", features = ["derive"] }
async-trait = "0.1.88"
```

### Basic web server

```rust
use sword::prelude::*;
use sword::web::HttpResult;

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[get("/")]
    async fn get_data() -> HttpResponse {
        let data = vec![
            "This is a basic web server",
            "It serves static data",
            "You can extend it with more routes",
        ];

        HttpResponse::Ok().data(data)
    }

    #[get("/hello")]
    async fn hello() -> HttpResponse {
        HttpResponse::Ok().data("Hello, World!")
    }

    #[post("/submit")]
    async fn submit_data(ctx: Context) -> HttpResult<HttpResponse> {
        let body = ctx.body::<serde_json::Value>()?;

        Ok(HttpResponse::Ok()
            .data(body)
            .message("Data submitted successfully"))
    }
}

#[sword::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Application::builder()?
        .controller::<AppController>()
        .run("0.0.0.0:8080")
        .await?;

    Ok(())
}
```

### With Middleware

```rust
use serde_json::json;
use sword::prelude::*;
use sword::web::HttpResult;

struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    async fn handle(mut ctx: Context, next: Next) -> MiddlewareResult {
        println!("Request: {} {}", ctx.method(), ctx.uri());

        ctx.extensions.insert::<String>("middleware_data".to_string());

        next!(ctx, next)
    }
}

#[controller("/api")]
struct AppController {}

#[routes]
impl AppController {
    #[get("/hello")]
    #[middleware(LoggingMiddleware)]
    async fn hello(ctx: Context) -> HttpResult<HttpResponse> {
        let middleware_data = ctx.extensions
            .get::<String>()
            .cloned()
            .unwrap_or_default();

        Ok(HttpResponse::Ok().data(json!({
            "message": "Hello from middleware!",
            "middleware_data": middleware_data
        })))
    }
}

#[sword::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Application::builder()?
        .controller::<AppController>()
        .run("0.0.0.0:8080")
        .await?;

    Ok(())
}
```

## More Examples

See the [examples directory](./examples) for more advanced usage.

## Hot reloading

In the case of use hot reloading, you need to install `dioxus-cli`:

```bash
cargo install --git https://github.com/DioxusLabs/dioxus.git dioxus-cli
```

Then run the server with:

```bash
dx serve --hot-patch --example hot_reloading
```
