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
- 💡 **Express-Like** - It provides a `Context` object with utility methods for request handling
- 💉 **Dependency Injection** - Built-in DI support using `shaku` crate

## 🛠️ Usage

### Basic web server 

```rust
use sword::prelude::*;
use sword::http::Result;

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
    async fn submit_data(ctx: Context) -> Result<HttpResponse> {
        let body = ctx.body::<serde_json::Value>()?;

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

### With State Management

```rust
use serde_json::json;
use std::sync::{Arc, OnceLock};
use sword::prelude::*;
use sword::http::Result;
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

#[controller("/api")]
struct AppController {}

#[controller_impl]
impl AppController {
    #[get("/data")]
    async fn get_data(ctx: Context) -> Result<HttpResponse> {
        let state = ctx.get_state::<AppState>()?;
        let count = state.db.read().await.len();
        let message = format!("Current data count: {}", count);

        state.db.write().await.push(message);

        Ok(HttpResponse::Ok().data(json!({
            "count": count,
            "current_data": state.db.read().await.clone(),
        })))
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

### With Middleware

```rust
use serde_json::json;
use sword::prelude::*;
use sword::http::Result;

struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    async fn handle(mut ctx: Context, next: Next) -> MiddlewareResult {
        println!("Request: {} {}", ctx.method(), ctx.uri());
        
        // Add some data to extensions
        ctx.extensions.insert::<String>("middleware_data".to_string());

        Ok(next.run(ctx.into()).await)
    }
}

#[controller("/api")]
struct AppController {}

#[controller_impl]
impl AppController {
    #[get("/hello")]
    #[middleware(LoggingMiddleware)]
    async fn hello(ctx: Context) -> Result<HttpResponse> {
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

#[tokio::main]
async fn main() {
    Application::builder()
        .controller::<AppController>()
        .run("0.0.0.0:8080")
        .await;
}
```

### With Dependency Injection

```rust
use std::sync::Arc;
use serde_json::json;
use shaku::{module, Component, Interface};
use sword::prelude::*;
use sword::http::Result;

trait Logger: Interface {
    fn log(&self, message: &str);
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("Log: {}", message);
    }
}

module! {
    AppModule {
        components = [ConsoleLogger],
        providers = []
    }
}

#[controller("/users")]
struct UserController {}

#[controller_impl]
impl UserController {
    #[get("/")]
    async fn get_users(ctx: Context) -> Result<HttpResponse> {
        let logger = ctx.get_dependency::<AppModule, dyn Logger>()?;
        logger.log("Fetching users");

        Ok(HttpResponse::Ok()
            .data(json!({
                "users": ["Alice", "Bob", "Charlie"]
            }))
            .message("Users retrieved successfully"))
    }
}

#[tokio::main]
async fn main() {
    let module = AppModule::builder().build();

    Application::builder()
        .di_module(Arc::new(module))
        .controller::<UserController>()
        .run("0.0.0.0:8080")
        .await;
}
```

### With Data Validation

```rust
use serde::{Deserialize, Serialize};
use sword::prelude::*;
use sword::http::Result;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
struct UserQuery {
    #[validate(range(message = "Page must be between 1 and 1000", min = 1, max = 1000))]
    page: u32,
    #[validate(range(message = "Limit must be between 1 and 100", min = 1, max = 100))]
    limit: u32,
}

#[derive(Serialize, Deserialize, Validate)]
struct CreateUserRequest {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    name: String,
    #[validate(email(message = "Invalid email format"))]
    email: String,
}

#[controller("/users")]
struct UserController {}

#[controller_impl]
impl UserController {
    #[get("/")]
    async fn get_users(ctx: Context) -> Result<HttpResponse> {
        let query = ctx.validated_query::<UserQuery>()?;
        
        Ok(HttpResponse::Ok()
            .data(format!("Page: {}, Limit: {}", query.page, query.limit))
            .message("Users retrieved successfully"))
    }

    #[post("/")]
    async fn create_user(ctx: Context) -> Result<HttpResponse> {
        let user = ctx.validated_body::<CreateUserRequest>()?;

        Ok(HttpResponse::Ok()
            .data(user)
            .message("User created successfully"))
    }
}

#[tokio::main]
async fn main() {
    Application::builder()
        .controller::<UserController>()
        .run("0.0.0.0:8080")
        .await;
}
```

### With Middleware Configuration

```rust
use serde_json::json;
use sword::prelude::*;
use sword::http::Result;

struct RoleMiddleware;

impl MiddlewareWithConfig<Vec<&str>> for RoleMiddleware {
    async fn handle(roles: Vec<&str>, mut ctx: Context, next: Next) -> MiddlewareResult {
        // Log the required roles
        println!("Required roles: {:?}", roles);
        
        // In a real application, you would validate the user's roles here
        // For this example, we'll just add the roles to the context
        ctx.extensions.insert(roles);

        Ok(next.run(ctx.into()).await)
    }
}

struct AuthenticationMiddleware;

impl MiddlewareWithConfig<String> for AuthenticationMiddleware {
    async fn handle(secret: String, mut ctx: Context, next: Next) -> MiddlewareResult {
        // In a real application, you would validate the JWT token here
        let auth_header = ctx.header("Authorization").unwrap_or("");
        
        if auth_header.is_empty() {
            return Ok(HttpResponse::Unauthorized()
                .message("Authorization header required")
                .into());
        }

        // Store the secret in extensions for demonstration
        ctx.extensions.insert(secret);
        
        Ok(next.run(ctx.into()).await)
    }
}

#[controller("/admin")]
struct AdminController {}

#[controller_impl]
impl AdminController {
    #[get("/users")]
    #[middleware(RoleMiddleware, config = vec!["admin", "user"])]
    async fn get_users(ctx: Context) -> Result<HttpResponse> {
        let roles = ctx.extensions
            .get::<Vec<&str>>()
            .cloned()
            .unwrap_or_default();

        Ok(HttpResponse::Ok()
            .data(json!({
                "users": ["Alice", "Bob", "Charlie"],
                "required_roles": roles
            }))
            .message("Users retrieved successfully"))
    }

    #[post("/protected")]
    #[middleware(AuthenticationMiddleware, config = "super-secret-key".to_string())]
    async fn protected_endpoint(ctx: Context) -> Result<HttpResponse> {
        let secret = ctx.extensions
            .get::<String>()
            .cloned()
            .unwrap_or_default();

        Ok(HttpResponse::Ok()
            .data(json!({
                "message": "Access granted to protected resource",
                "secret_used": !secret.is_empty()
            })))
    }
}

#[tokio::main]
async fn main() {
    Application::builder()
        .controller::<AdminController>()
        .run("0.0.0.0:8080")
        .await;
}
```

## Currently working on
- ✅📱 Add Application struct
- ✅ 🏗️ Add Application Context
- ✅ 🔒 Add Middleware support
- ✅ 💉 Add Dependency Injection support based on `shaku` crate

## 📋 Roadmap

- [ ] ⚙️ Add config file support
- [ ] 📁 Add File - FormData support
- [ ] 🧪 Add more tests
- [ ] 📚 Add more documentation
- [ ] 🛠️ CLI Command line interface for code-generation (templates)


