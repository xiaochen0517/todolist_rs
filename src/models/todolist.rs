use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct TodoListPageRequest {
    pub page: i32,
    pub page_size: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoListPageResponse {
    pub test: i32,
}
