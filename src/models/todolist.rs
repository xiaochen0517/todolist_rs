use rocket_validation::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromForm, Validate)]
pub struct TodoListPageRequest {
    #[validate(range(min = 1, message = "页码必须大于等于 1"))]
    pub page: i32,
    pub page_size: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoListPageResponse {
    pub test: i32,
}
