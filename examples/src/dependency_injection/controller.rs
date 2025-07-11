use serde::{Deserialize, Serialize};
use serde_json::json;
use sword::{prelude::*, web::HttpResult};
use validator::Validate;

use crate::{database::DataRepository, logger::Logger, AppModule};

#[derive(Serialize, Deserialize, Validate)]
struct IncommingUser {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Name must be between 1 and 20 characters long"
    ))]
    name: String,
}

#[controller("/users")]
pub struct UserController {}

#[routes]
impl UserController {
    #[get("/")]
    async fn get_users(ctx: Context) -> HttpResult<HttpResponse> {
        let repository = ctx.get_dependency::<AppModule, dyn DataRepository>()?;
        let users = repository.get_data().await;

        ctx.get_dependency::<AppModule, dyn Logger>()?
            .log(&format!("Found {} users", users.len()));

        Ok(HttpResponse::Ok().data(json!({ "users": repository.get_data().await })))
    }

    #[post("/")]
    async fn add_user(ctx: Context) -> HttpResult<HttpResponse> {
        let user: IncommingUser = ctx.validated_body()?;
        let repository = ctx.get_dependency::<AppModule, dyn DataRepository>()?;

        repository.add_data(user.name).await;

        ctx.get_dependency::<AppModule, dyn Logger>()?
            .log("User added successfully");

        Ok(HttpResponse::Created().data("User added successfully"))
    }
}
