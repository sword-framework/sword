use axum_test::{
    TestServer,
    multipart::{MultipartForm, Part},
};

use std::fs;
use sword::prelude::*;

use crate::utils::TempFile;

#[controller("/")]
struct TestController {}

#[routes]
impl TestController {
    #[post("/multipart")]
    async fn hello(ctx: Context) -> HttpResult<HttpResponse> {
        let form = ctx.multipart().await?;
        let mut fields: Vec<String> = Vec::new();

        for field in form.fields() {
            fields.push(field.name.clone().unwrap_or("unknown".to_string()));
        }

        Ok(HttpResponse::Ok().data(fields).message("Hello, Multipart!"))
    }
}

#[tokio::test]
async fn exceed_limit() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.with_controller::<TestController>().build();

    let test = TestServer::new(app.router()).unwrap();

    let temp_file = TempFile::with_size(1024 * 1024 * 2); // 2 MB
    let bytes = fs::read(&temp_file.path).expect("Failed to read test file");

    let part = Part::bytes(bytes).file_name("large_test_file.txt").mime_type("text/plain");

    let form = MultipartForm::new().add_text("field1", "value1").add_part("file", part);

    let response = test.post("/multipart").multipart(form).await;
    let json = response.json::<ResponseBody>();

    assert_eq!(response.status_code(), 413);

    assert_eq!(json.message, "Request payload too large".into());
    assert_eq!(json.data["type"], "PayloadTooLarge");

    Ok(())
}

/// Tests that a file exactly at the body limit (considering multipart overhead) is accepted.
/// The effective limit is ~975KB due to multipart headers/boundaries overhead.
#[tokio::test]
async fn body_limit_exactly_at_limit() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.with_controller::<TestController>().build();

    let test = TestServer::new(app.router()).unwrap();

    // Based on testing, the effective limit is around 975 KB (considering multipart overhead)
    let temp_file = TempFile::with_size(975 * 1024); // 975 KB - right at the boundary
    let bytes = fs::read(&temp_file.path).expect("Failed to read test file");

    let part = Part::bytes(bytes)
        .file_name("exactly_at_limit_file.txt")
        .mime_type("text/plain");

    let form = MultipartForm::new().add_text("field1", "value1").add_part("file", part);

    let response = test.post("/multipart").multipart(form).await;

    assert_eq!(response.status_code(), 200);
    let json = response.json::<ResponseBody>();
    assert_eq!(json.message, "Hello, Multipart!".into());

    Ok(())
}

#[tokio::test]
async fn body_limit_just_under_limit() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.with_controller::<TestController>().build();

    let test = TestServer::new(app.router()).unwrap();

    // Create a file just under the effective limit
    let temp_file = TempFile::with_size(970 * 1024); // 970 KB - safely under the limit
    let bytes = fs::read(&temp_file.path).expect("Failed to read test file");

    let part = Part::bytes(bytes)
        .file_name("just_under_limit_file.txt")
        .mime_type("text/plain");

    let form = MultipartForm::new().add_text("field1", "value1").add_part("file", part);

    let response = test.post("/multipart").multipart(form).await;

    assert_eq!(response.status_code(), 200);
    let json = response.json::<ResponseBody>();
    assert_eq!(json.message, "Hello, Multipart!".into());

    Ok(())
}

#[tokio::test]
async fn body_limit_just_over_limit() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.with_controller::<TestController>().build();

    let test = TestServer::new(app.router()).unwrap();

    // Create a file just over the effective limit
    let temp_file = TempFile::with_size(976 * 1024); // 976 KB - just over the limit
    let bytes = fs::read(&temp_file.path).expect("Failed to read test file");

    let part = Part::bytes(bytes).file_name("just_over_limit_file.txt").mime_type("text/plain");

    let form = MultipartForm::new().add_text("field1", "value1").add_part("file", part);

    let response = test.post("/multipart").multipart(form).await;

    assert_eq!(response.status_code(), 413);
    let json = response.json::<ResponseBody>();
    assert_eq!(json.message, "Request payload too large".into());
    assert_eq!(json.data["type"], "PayloadTooLarge");

    Ok(())
}

#[tokio::test]
async fn body_limit_multiple_fields_exceed_limit() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.with_controller::<TestController>().build();

    let test = TestServer::new(app.router()).unwrap();

    // Create multiple smaller files that together exceed the limit
    let temp_file1 = TempFile::with_size(700 * 1024); // 700 KB
    let temp_file2 = TempFile::with_size(700 * 1024); // 700 KB
    // Total: ~1.4 MB plus multipart overhead should exceed 1MB limit

    let bytes1 = fs::read(&temp_file1.path).expect("Failed to read test file 1");
    let bytes2 = fs::read(&temp_file2.path).expect("Failed to read test file 2");

    let part1 = Part::bytes(bytes1).file_name("file1.txt").mime_type("text/plain");

    let part2 = Part::bytes(bytes2).file_name("file2.txt").mime_type("text/plain");

    let form = MultipartForm::new()
        .add_text("field1", "value1")
        .add_part("file1", part1)
        .add_part("file2", part2);

    let response = test.post("/multipart").multipart(form).await;

    assert_eq!(response.status_code(), 413);
    let json = response.json::<ResponseBody>();
    assert_eq!(json.message, "Request payload too large".into());
    assert_eq!(json.data["type"], "PayloadTooLarge");

    Ok(())
}

/// Tests that multiple files that together stay within the body limit are accepted.
#[tokio::test]
async fn body_limit_small_fields_within_limit() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.with_controller::<TestController>().build();

    let test = TestServer::new(app.router()).unwrap();

    // Create multiple smaller files that together stay within the limit
    let temp_file1 = TempFile::with_size(300 * 1024); // 300 KB
    let temp_file2 = TempFile::with_size(300 * 1024); // 300 KB
    // Total: ~600 KB plus multipart overhead should be within 1MB limit

    let bytes1 = fs::read(&temp_file1.path).expect("Failed to read test file 1");
    let bytes2 = fs::read(&temp_file2.path).expect("Failed to read test file 2");

    let part1 = Part::bytes(bytes1).file_name("file1.txt").mime_type("text/plain");

    let part2 = Part::bytes(bytes2).file_name("file2.txt").mime_type("text/plain");

    let form = MultipartForm::new()
        .add_text("field1", "value1")
        .add_text("field2", "value2")
        .add_part("file1", part1)
        .add_part("file2", part2);

    let response = test.post("/multipart").multipart(form).await;

    assert_eq!(response.status_code(), 200);
    let json = response.json::<ResponseBody>();
    assert_eq!(json.message, "Hello, Multipart!".into());

    Ok(())
}

#[tokio::test]
async fn valid_mime() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.with_controller::<TestController>().build();

    let test = TestServer::new(app.router()).unwrap();

    let bytes = include_bytes!("../../files/pdf-test.pdf").to_vec();

    let part = Part::bytes(bytes).file_name("pdf-test.pdf").mime_type("application/pdf");

    let form = MultipartForm::new().add_text("field1", "value1").add_part("file", part);

    let response = test.post("/multipart").multipart(form).await;

    assert_eq!(response.status_code(), 200);
    let json = response.json::<ResponseBody>();

    assert_eq!(json.message, "Hello, Multipart!".into());

    Ok(())
}

#[tokio::test]
async fn invalid_mime() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.with_controller::<TestController>().build();

    let test = TestServer::new(app.router()).unwrap();

    let bytes = include_bytes!("../../files/png-test.png").to_vec();

    let part = Part::bytes(bytes).file_name("png-test.png").mime_type("application/pdf");

    let form = MultipartForm::new().add_text("field1", "value1").add_part("file", part);

    let response = test.post("/multipart").multipart(form).await;

    assert_eq!(response.status_code(), 415);
    let json = response.json::<ResponseBody>();

    assert_eq!(json.message, "Unsupported media type".into());

    Ok(())
}
