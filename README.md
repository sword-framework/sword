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
sword = "0.1.6"
```

### Other useful dependencies

```toml
# Data serialization and deserialization
serde = { version = "*", features = ["derive"] }

# JSON data handling
serde_json = "*"

# Data validation and schema definition
validator = { version = "*", features = ["derive"] }
```

### Basic web server

```rust
use sword::prelude::*;
use serde_json::Value;

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
        let body = ctx.body::<Value>()?;

        Ok(HttpResponse::Ok()
            .data(body)
            .message("Data submitted successfully"))
    }
}

#[sword::main]
async fn main() {
    let app = Application::builder()?
        .with_controller::<AppController>()
        .build();

    app.run().await?;
}
```

### With Middleware

```rust
use serde_json::json;
use sword::prelude::*;

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
async fn main() {
    let app = Application::builder()?
        .with_controller::<AppController>()
        .build();

    app.run().await?;
}
```

## More Examples

See the [examples directory](./examples) for more advanced usage.

## Hot reloading

In the case of use hot reloading, you need to install `dioxus-cli`:

```bash
cargo install --git https://github.com/DioxusLabs/dioxus.git dioxus-cli
```

Then run the hot reload example server with:

```bash
cd examples
dx serve --hot-patch -p hot_reload
```
