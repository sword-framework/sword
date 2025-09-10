use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct MyQuery {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct MyBody {
    #[validate(length(min = 1, message = "Content must not be empty"))]
    pub content: String,
}
