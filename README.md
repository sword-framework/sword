<div align="center">
<img src="https://pillan.inf.uct.cl/~lrevillod/images/sword-logo.webp" alt="Sword Logo" width="200">

<h1>⚔️ Sword ⚔️</h1>
<p><em>A prototype for a rust web framework</em></p>

---

> **🚧 Prototype Status**
> 
> This is a **prototype** and **not production-ready**. It is intended for:
> - 📚 **Educational purposes**
> - 🔬 **Exploring web framework design**
> - 🛠️ **Axum wrapper experimentation**
> 
> While it may scale well in the future, **it is not recommended for production use** at this stage.

---

</div>

## ✨ Features

- 🛣️ **Macro-based routing** - Clean and intuitive route definitions
- 🔍 **Complex query parameters** - Ready for advanced parameter handling
- 📄 **JSON-first design** - Built with JSON formats as priority
- ✅ **Built-in validation** - Support with `serde` and `validator` crates
- 🌐 **RFC-compliant HTTP responses** - Using `axum_responses` crate
- 💡 **Express-Like** - It provides a `req` from request with some utility methods.

## 🛠️ Usage

### Basic web server 

```rust
use sword::prelude::*;

#[controller("/")]
struct AppController {}

#[controller_impl]
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
    async fn submit_data(req: Request) -> Result<HttpResponse> {
        let body = req.body::<serde_json::Value>()?;

        Ok(HttpResponse::Ok()
            .data(body)
            .message("Data submitted successfully"))
    }
}

#[tokio::main]
async fn main() {
    Application::builder()
        .controller::<AppController>()
        .run("0.0.0.0:8080")
        .await;
}

```

### With Middleware

```rust
use serde_json::json;
use std::sync::{Arc, OnceLock};
use sword::prelude::*;
use tokio::sync::RwLock;

type InMemoryDb = Arc<RwLock<Vec<String>>>;
const IN_MEMORY_DB: OnceLock<InMemoryDb> = OnceLock::new();

fn db() -> Arc<RwLock<Vec<String>>> {
    IN_MEMORY_DB
        .get_or_init(|| Arc::new(RwLock::new(Vec::new())))
        .clone()
}

#[derive(Clone)]
struct AppState {
    db: InMemoryDb,
}

struct MyMiddleware;

impl MiddlewareWithState<AppState> for MyMiddleware {
    async fn handle(ctx: State<AppState>, mut req: Request, next: Next) -> MiddlewareResult {
        let count = ctx.db.read().await.len();
        req.extensions.insert(count);

        Ok(next.run(req.into()).await)
    }
}

#[controller("/api")]
struct AppController {}

#[controller_impl]
impl AppController {
    #[get("/data")]
    #[middleware(MyMiddleware)]
    async fn submit_data(state: State<AppState>, req: Request) -> HttpResponse {
        let db = &state.db;
        let count = req.extensions.get::<usize>().cloned().unwrap_or(0);
        let message = format!("Current data count: {}", count);

        db.write().await.push(message);

        HttpResponse::Ok().data(json!({
            "count": count,
            "current_data": db.read().await.clone(),
        }))
    }
}

#[tokio::main]
async fn main() {
    let app_state = AppState { db: db() };

    Application::builder()
        .state(app_state)
        .controller::<AppController>()
        .run("0.0.0.0:8080")
        .await;
}

```

## Currently working on
- ✅📱 Add Application struct
- ✅ 🏗️ Add Application Context
- ✅ 🔒 Add Middleware support
- [ ] 💉 Add Dependency Injection support based on `shaku` crate

## 📋 Roadmap

- [ ] ⚙️ Add config file support
- [ ] 📁 Add File - FormData support
- [ ] 🧪 Add more tests
- [ ] 📚 Add more documentation
- [ ] 🛠️ CLI Command line interface for code-generation (templates)


