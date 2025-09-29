use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Validate)]
pub struct MyBody {
    #[garde(length(min = 1))]
    pub content: String,
}
