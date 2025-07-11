use std::sync::{Arc, OnceLock};

use axum_test::TestServer;
use serde::{Deserialize, Serialize};
use sword::prelude::*;
use sword::web::HttpResult;
use validator::Validate;

pub static APP: OnceLock<Arc<TestServer>> = OnceLock::new();

#[cfg(test)]
fn test_server() -> Result<Arc<TestServer>, Box<dyn std::error::Error>> {
    use sword::application::Application;

    let app = Application::builder()?.controller::<UserController>();

    Ok(APP
        .get_or_init(|| Arc::new(TestServer::new(app.router()).unwrap()))
        .clone())
}

#[derive(Deserialize, Serialize)]
struct Data {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct QueryData {
    page: Option<u32>,
    limit: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
struct ValidableQueryData {
    #[validate(range(message = "Page must be between 1 and 1000", min = 1, max = 1000))]
    page: u32,
    #[validate(range(message = "Limit must be between 1 and 100", min = 1, max = 100))]
    limit: u32,
}

#[controller("/users")]
pub struct UserController {}

#[routes]
impl UserController {
    #[get("/simple-query")]
    async fn get_users(ctx: Context) -> HttpResult<HttpResponse> {
        let query: QueryData = ctx.query()?;

        Ok(HttpResponse::Ok()
            .data(query)
            .message("Users retrieved successfully"))
    }

    #[get("/validate-query")]
    async fn get_users_with_validation(ctx: Context) -> HttpResult<HttpResponse> {
        let query: ValidableQueryData = ctx.validated_query()?;

        Ok(HttpResponse::Ok()
            .data(query)
            .message("Users retrieved successfully with validation"))
    }
}

#[tokio::test]
async fn unvalidated_query_test() -> Result<(), Box<dyn std::error::Error>> {
    use sword::web::ResponseBody;

    let app = test_server()?;
    let response = app.get("/users/simple-query?page=1&limit=5").await;

    let json = response.json::<ResponseBody>();

    assert_eq!(200_u16, response.status_code().as_u16());
    assert!(json.data.get("page").is_some());
    assert!(json.data.get("limit").is_some());

    assert_eq!(json.data.get("page").unwrap(), "1".parse::<u32>().unwrap());
    assert_eq!(json.data.get("limit").unwrap(), "5".parse::<u32>().unwrap());

    Ok(())
}

#[tokio::test]
async fn validated_query_test() -> Result<(), Box<dyn std::error::Error>> {
    use sword::web::ResponseBody;

    let app = test_server()?;
    let response = app.get("/users/validate-query?page=1&limit=5").await;

    let json = response.json::<ResponseBody>();

    assert_eq!(200_u16, response.status_code().as_u16());
    assert!(json.data.get("page").is_some());
    assert!(json.data.get("limit").is_some());

    assert_eq!(json.data.get("page").unwrap(), "1".parse::<u32>().unwrap());
    assert_eq!(json.data.get("limit").unwrap(), "5".parse::<u32>().unwrap());

    Ok(())
}

#[tokio::test]
async fn validated_query_error_test() -> Result<(), Box<dyn std::error::Error>> {
    use sword::web::ResponseBody;

    let app = test_server()?;
    let response = app.get("/users/validate-query?page=1001&limit=5").await;

    let json = response.json::<ResponseBody>();

    assert_eq!(400_u16, response.status_code().as_u16());

    assert!(json.data.get("type").is_some());
    assert_eq!(json.data.get("type").unwrap(), "ValidationError");
    assert!(json.data.get("errors").is_some());

    let errors = json.data.get("errors").unwrap().as_array().unwrap();

    assert_eq!(errors.len(), 1);

    let error = &errors[0];

    assert!(error.get("field").is_some());
    assert_eq!(error.get("field").unwrap(), "page");
    assert!(error.get("message").is_some());
    assert_eq!(
        error.get("message").unwrap(),
        "Page must be between 1 and 1000"
    );

    Ok(())
}
