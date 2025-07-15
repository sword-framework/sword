use axum_test::{
    TestServer,
    multipart::{MultipartForm, Part},
};

use serde_json::Value;
use std::fs;
use sword::prelude::*;

#[controller("/")]
struct TestController {}

#[routes]
impl TestController {
    #[post("/multipart")]
    async fn hello(ctx: Context) -> HttpResult<HttpResponse> {
        let form = ctx.multipart().await?;
        let mut fields: Vec<String> = Vec::new();

        for field in form.iter() {
            fields.push(field.name.clone().unwrap_or("unknown".to_string()));
        }

        Ok(HttpResponse::Ok().data(fields).message("Hello, Multipart!"))
    }
}

#[tokio::test]
async fn test_application() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.controller::<TestController>();
    let test = TestServer::new(app.router()).unwrap();

    let file = create_file_with_size(1024 * 1024 / 2);
    let bytes = fs::read(&file).expect("Failed to read test file");

    let part = Part::bytes(bytes)
        .file_name("test_file.txt")
        .mime_type("text/plain");

    let form = MultipartForm::new()
        .add_text("field1", "value1")
        .add_text("field2", "value2")
        .add_part("file", part);

    let response = test.post("/multipart").multipart(form).await;
    let json = response.json::<ResponseBody>();

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        json.data,
        Value::Array(vec![
            Value::String("field1".to_string()),
            Value::String("field2".to_string()),
            Value::String("file".to_string()),
        ])
    );

    fs::remove_file(file).expect("Failed to remove test file");

    Ok(())
}

fn create_file_with_size(size: usize) -> String {
    let file_path = "./test_file.txt";
    let content = vec![b'x'; size];
    fs::write(file_path, content).expect("Failed to write test file");
    file_path.to_string()
}
