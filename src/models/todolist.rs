use crate::db::todolist;
use rocket::request::FromRequest;
use rocket_validation::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromForm, Validate)]
pub struct TodoListPageRequest {
    #[validate(range(min = 1, message = "页码必须大于等于 1"))]
    pub page: u64,
    #[validate(range(min = 1, message = "分页数量必须大于等于 1"))]
    pub page_size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoListPageResponse {
    pub list: Vec<todolist::Model>,
    pub current_page: u64,
    pub page_size: u64,
    pub total_pages: u64,
    pub total_size: u64,
}
