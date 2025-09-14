
# Sword
> <img src="https://avatars.githubusercontent.com/u/228345998?s=200&v=4" align="right" width="120"/>

Structured web framework for rust built on top of axum.  
Designed to build server application with less boilerplate and more simplicity.  
It takes advantage of the tokio ecosystem to bring you performance with nice DX.

## Features

- **Macro-based routing** - Clean and intuitive route definitions
- **JSON-first design** - Built with JSON formats as priority
- **Built-in validation** - Support with `serde` and `validator` crates
- **RFC-compliant HTTP responses** - Using `axum_responses` crate
- **Express-Like** - It provides a `Context` object with utility methods for request handling
- **Dependency Injection** - Built-in DI support using `shaku` crate
- **Middleware support** - Easily add middleware to routes or controllers
- **Asynchronous by default** - Built on top of `axum` and `tokio`

## Usage

### Add to your `Cargo.toml`

```toml
[dependencies]
sword = "0.1.7"
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
    #[get("/hello")]
    async fn hello() -> HttpResponse {
        HttpResponse::Ok().message("Hello, World!")
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

## More Examples

See the [examples directory](./examples) for more advanced usage.

## Hot reloading

In the case of use hot reloading, you need to install `dioxus-cli`:

```bash
cargo install --git https://github.com/DioxusLabs/dioxus.git dioxus-cli
```

See the [hot reloading example](./examples/hot-reload) for more details.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for more details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request. See [CONTRIBUTING.md](./CONTRIBUTING.md) for more details.
