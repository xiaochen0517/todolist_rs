use crate::db::prelude::Todolist;
use crate::db::{todolist, user};
use crate::models::todolist::{TodoListPageRequest, TodoListPageResponse};
use crate::utils::response::{ApiError, ApiResult, ErrorInfo};
use rocket::serde::json::Json;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait};

pub struct TodolistService {}

fn handle_db_error(e: sea_orm::DbErr) -> ApiError<ErrorInfo<()>> {
    error!("Database error: {}", e);
    ApiError::create_internal_error(ErrorInfo::DATABASE_ERROR_CODE, "Database operation failed")
}

impl TodolistService {
    pub async fn get(
        db: &DatabaseConnection,
        user_info: user::Model,
        todolist_page_request: &TodoListPageRequest,
    ) -> ApiResult<TodoListPageResponse, ErrorInfo<()>> {
        let paginator = user_info
            .find_related(Todolist)
            .paginate(db, todolist_page_request.page_size);
        let items_and_pages_number = paginator
            .num_items_and_pages()
            .await
            .map_err(handle_db_error)?;
        // 如果当前查询页码大于总页数，修改为最大页数
        let mut current_page = todolist_page_request.page - 1;
        if current_page >= items_and_pages_number.number_of_pages {
            current_page = items_and_pages_number.number_of_pages - 1;
        }
        let todolist_vec = user_info
            .find_related(Todolist)
            .paginate(db, todolist_page_request.page_size)
            .fetch_page(current_page)
            .await
            .map_err(handle_db_error)?;
        debug!("todolist page: {:?}", todolist_vec);
        Ok(Json(TodoListPageResponse {
            list: todolist_vec,
            current_page: current_page + 1,
            page_size: todolist_page_request.page_size,
            total_pages: items_and_pages_number.number_of_pages,
            total_size: items_and_pages_number.number_of_items,
        }))
    }
}
