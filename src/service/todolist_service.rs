use sea_orm::DatabaseConnection;
use crate::db::user;
use crate::models::todolist::TodoListPageResponse;
use crate::utils::response::{ApiResult, ErrorInfo};

pub struct TodolistService {}

impl TodolistService {
    pub async fn get(db: &DatabaseConnection, user_info: user::Model) -> ApiResult<TodoListPageResponse, ErrorInfo<()>> {
        todo!()
    }
}
