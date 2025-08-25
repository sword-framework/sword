use axum_test::{
    TestServer,
    multipart::{MultipartForm, Part},
};

use std::fs;
use sword::prelude::*;

pub fn create_file_with_size(size: usize) -> String {
    let file_path = "./test_file.txt";
    let content = vec![b'x'; size];
    std::fs::write(file_path, content).expect("Failed to write test file");
    file_path.to_string()
}

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
async fn exceed_limit() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.controller::<TestController>();
    let test = TestServer::new(app.router()).unwrap();

    let file = create_file_with_size(1024 * 1024 * 2); // 2 MB
    let bytes = fs::read(&file).expect("Failed to read test file");

    let part = Part::bytes(bytes)
        .file_name("large_test_file.txt")
        .mime_type("text/plain");

    let form = MultipartForm::new()
        .add_text("field1", "value1")
        .add_part("file", part);

    let response = test.post("/multipart").multipart(form).await;

    let json = response.json::<ResponseBody>();

    assert_eq!(response.status_code(), 400);

    assert_eq!(
        json.data["details"],
        "Error reading body: length limit exceeded".to_string()
    );

    fs::remove_file(file).expect("Failed to remove test file");

    Ok(())
}

#[tokio::test]
async fn valid_mime() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.controller::<TestController>();
    let test = TestServer::new(app.router()).unwrap();

    let bytes = include_bytes!("../../files/pdf-test.pdf").to_vec();

    let part = Part::bytes(bytes)
        .file_name("pdf-test.pdf")
        .mime_type("application/pdf");

    let form = MultipartForm::new()
        .add_text("field1", "value1")
        .add_part("file", part);

    let response = test.post("/multipart").multipart(form).await;

    assert_eq!(response.status_code(), 200);
    let json = response.json::<ResponseBody>();

    assert_eq!(json.message, "Hello, Multipart!".into());

    Ok(())
}

#[tokio::test]
async fn invalid_mime() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.controller::<TestController>();
    let test = TestServer::new(app.router()).unwrap();

    let bytes = include_bytes!("../../files/png-test.png").to_vec();

    let part = Part::bytes(bytes)
        .file_name("png-test.png")
        .mime_type("application/pdf");

    let form = MultipartForm::new()
        .add_text("field1", "value1")
        .add_part("file", part);

    let response = test.post("/multipart").multipart(form).await;

    assert_eq!(response.status_code(), 415);
    let json = response.json::<ResponseBody>();

    assert_eq!(json.message, "Unsupported media type".into());

    Ok(())
}
