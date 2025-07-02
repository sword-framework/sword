use axum_test::TestServer;
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use shaku::{Component, Interface, module};
use sword::{http::Result, prelude::*};

trait CounterService: Interface {
    fn get_count(&self) -> usize;
    fn increment(&self);
    fn add(&self, value: usize);
    fn reset(&self);
}

#[derive(Component)]
#[shaku(interface = CounterService)]
struct AtomicCounter {
    #[shaku(default)]
    count: Arc<AtomicUsize>,
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self {
            count: Arc::new(AtomicUsize::new(0)),
        }
    }
}

#[allow(dead_code)]
impl AtomicCounter {
    fn new() -> Self {
        Self::default()
    }
}

impl CounterService for AtomicCounter {
    fn get_count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    fn increment(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    fn add(&self, value: usize) {
        self.count.fetch_add(value, Ordering::SeqCst);
    }

    fn reset(&self) {
        self.count.store(0, Ordering::SeqCst);
    }
}

trait Logger: Interface {
    fn log(&self, message: &str);
    fn get_logs(&self) -> Vec<String>;
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct InMemoryLogger {
    #[shaku(default)]
    logs: Arc<std::sync::Mutex<Vec<String>>>,
}

impl Default for InMemoryLogger {
    fn default() -> Self {
        Self {
            logs: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
}

#[allow(dead_code)]
impl InMemoryLogger {
    fn new() -> Self {
        Self::default()
    }
}

impl Logger for InMemoryLogger {
    fn log(&self, message: &str) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.push(message.to_string());
        }
    }

    fn get_logs(&self) -> Vec<String> {
        if let Ok(logs) = self.logs.lock() {
            logs.clone()
        } else {
            Vec::new()
        }
    }
}

module! {
    TestModule {
        components = [AtomicCounter, InMemoryLogger],
        providers = []
    }
}

#[controller("/api")]
struct TestController {}

#[controller_impl]
impl TestController {
    #[get("/counter")]
    async fn get_counter(ctx: Context) -> Result<HttpResponse> {
        let counter_service = ctx.get_dependency::<TestModule, dyn CounterService>()?;
        let logger = ctx.get_dependency::<TestModule, dyn Logger>()?;

        let count = counter_service.get_count();
        logger.log(&format!("Counter accessed: {}", count));

        Ok(HttpResponse::Ok()
            .data(json!({ "count": count }))
            .message("Counter retrieved successfully"))
    }

    #[post("/counter/increment")]
    async fn increment_counter(ctx: Context) -> Result<HttpResponse> {
        ctx.get_dependency::<TestModule, dyn Logger>()?
            .log("Incrementing counter");

        let counter_service = ctx.get_dependency::<TestModule, dyn CounterService>()?;

        counter_service.increment();

        let count = counter_service.get_count();

        ctx.get_dependency::<TestModule, dyn Logger>()?
            .log(&format!("Counter incremented to: {}", count));

        Ok(HttpResponse::Ok()
            .data(json!({ "count": count }))
            .message("Counter incremented successfully"))
    }

    #[post("/counter/add")]
    async fn add_to_counter(ctx: Context) -> Result<HttpResponse> {
        #[derive(serde::Deserialize)]
        struct AddRequest {
            value: usize,
        }

        let body: AddRequest = ctx.body()?;
        let logger = ctx.get_dependency::<TestModule, dyn Logger>()?;
        let counter_service = ctx.get_dependency::<TestModule, dyn CounterService>()?;

        counter_service.add(body.value);

        let count = counter_service.get_count();

        logger.log(&format!(
            "Added {} to counter, new value: {}",
            body.value, count
        ));

        Ok(HttpResponse::Ok()
            .data(json!({
                "count": count,
                "added": body.value
            }))
            .message("Value added to counter successfully"))
    }

    #[get("/logs")]
    async fn get_logs(ctx: Context) -> Result<HttpResponse> {
        let logger = ctx.get_dependency::<TestModule, dyn Logger>()?;
        let logs = logger.get_logs();

        Ok(HttpResponse::Ok()
            .data(json!({ "logs": logs }))
            .message("Logs retrieved successfully"))
    }

    #[post("/counter/reset")]
    async fn reset_counter(ctx: Context) -> Result<HttpResponse> {
        ctx.get_dependency::<TestModule, dyn Logger>()?
            .log("Resetting counter to 0");

        ctx.get_dependency::<TestModule, dyn CounterService>()?
            .reset();

        Ok(HttpResponse::Ok()
            .data(json!({ "count": 0 }))
            .message("Counter reset successfully"))
    }
}

#[tokio::test]
async fn test_dependency_injection_with_multiple_services() {
    let module = TestModule::builder().build();

    let app = Application::builder()
        .di_module(module)
        .controller::<TestController>();

    let server = TestServer::new(app.router()).unwrap();

    let get_response = server.get("/api/counter").await;
    assert_eq!(get_response.status_code(), 200);
    let get_json = get_response.json::<ResponseBody>();
    assert_eq!(get_json.data.unwrap()["count"], 0);

    let increment_response = server.post("/api/counter/increment").await;
    assert_eq!(increment_response.status_code(), 200);
    let increment_json = increment_response.json::<ResponseBody>();
    assert_eq!(increment_json.data.unwrap()["count"], 1);

    let add_response = server
        .post("/api/counter/add")
        .json(&json!({ "value": 5 }))
        .await;
    assert_eq!(add_response.status_code(), 200);
    let add_json = add_response.json::<ResponseBody>();
    let add_data = add_json.data.unwrap();
    assert_eq!(add_data["count"], 6);
    assert_eq!(add_data["added"], 5);

    let logs_response = server.get("/api/logs").await;
    assert_eq!(logs_response.status_code(), 200);
    let logs_json = logs_response.json::<ResponseBody>();
    let logs_data = logs_json.data.unwrap();
    let logs = logs_data["logs"].as_array().unwrap();

    assert_eq!(logs.len(), 4);
    assert!(logs[0].as_str().unwrap().contains("Counter accessed: 0"));
    assert!(logs[1].as_str().unwrap().contains("Incrementing counter"));
    assert!(
        logs[2]
            .as_str()
            .unwrap()
            .contains("Counter incremented to: 1")
    );
    assert!(
        logs[3]
            .as_str()
            .unwrap()
            .contains("Added 5 to counter, new value: 6")
    );

    let reset_response = server.post("/api/counter/reset").await;
    assert_eq!(reset_response.status_code(), 200);
    let reset_json = reset_response.json::<ResponseBody>();
    assert_eq!(reset_json.data.unwrap()["count"], 0);

    let final_response = server.get("/api/counter").await;
    assert_eq!(final_response.status_code(), 200);
    let final_json = final_response.json::<ResponseBody>();
    assert_eq!(final_json.data.unwrap()["count"], 0);
}

#[tokio::test]
async fn test_service_isolation_between_tests() {
    let module = TestModule::builder().build();

    let app = Application::builder()
        .di_module(module)
        .controller::<TestController>();

    let server = TestServer::new(app.router()).unwrap();

    let count_response = server.get("/api/counter").await;
    assert_eq!(count_response.status_code(), 200);
    let count_json = count_response.json::<ResponseBody>();
    assert_eq!(count_json.data.unwrap()["count"], 0);

    let logs_response = server.get("/api/logs").await;
    assert_eq!(logs_response.status_code(), 200);
    let logs_json = logs_response.json::<ResponseBody>();
    let logs_data = logs_json.data.unwrap();
    let logs = logs_data["logs"].as_array().unwrap();

    assert_eq!(logs.len(), 1);
}
