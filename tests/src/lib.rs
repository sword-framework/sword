use sword::http::{HttpResponse, Request, Result};
use sword::routing::{get, post, router};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Data {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct QueryData {
    page: Option<u32>,
    limit: Option<u32>,
}

pub struct UserController;

#[router("/users")]
impl UserController {
    #[get("/")]
    async fn get_users(req: Request) -> Result<HttpResponse> {
        let query: QueryData = req.query()?;

        Ok(HttpResponse::Ok()
            .data(query)
            .message("Users retrieved successfully"))
    }

    #[get("/{id}")]
    async fn get_user_by_id(req: Request) -> Result<HttpResponse> {
        let _ = req.param::<String>("id")?;

        Ok(HttpResponse::Ok()
            .data("Hello, World!")
            .message("Request was successful"))
    }

    #[post("/")]
    async fn post_handler(req: Request) -> Result<HttpResponse> {
        let body = req.body::<Data>()?;

        Ok(HttpResponse::Created()
            .data(body)
            .message("User created successfully"))
    }
}

#[tokio::test]
async fn query_test() {
    use sword::http::ResponseBody;

    let router = UserController::router();

    let app = axum_test::TestServer::new(router).unwrap();
    let response = app.get("/users?page=1&limit=5").await;

    let json = response.json::<ResponseBody>();

    let Some(data) = json.data else {
        panic!("Expected data in response");
    };

    assert_eq!(200_u16, response.status_code().as_u16());
    assert!(data.get("page").is_some());
    assert!(data.get("limit").is_some());

    assert_eq!(data.get("page").unwrap(), "1".parse::<u32>().unwrap());
    assert_eq!(data.get("limit").unwrap(), "5".parse::<u32>().unwrap());
}

#[tokio::test]
async fn url_param_test() {
    let router = UserController::router();

    let app = axum_test::TestServer::new(router).unwrap();
    let response = app.get("/users/abc").await;

    assert_eq!(200_u16, response.status_code().as_u16())
}

#[tokio::test]
async fn post_data_test() {
    use sword::http::ResponseBody;

    let router = UserController::router();

    let app = axum_test::TestServer::new(router).unwrap();
    let response = app
        .post("/users")
        .json(&Data {
            name: "John Doe".to_string(),
        })
        .add_header("Content-Type", "application/json")
        .await;

    let data = response.json::<ResponseBody>();

    let Some(data) = data.data else {
        panic!("Expected data in response");
    };

    assert_eq!(201_u16, response.status_code().as_u16());
    assert_eq!(data.get("name").unwrap(), "John Doe");
}
