use crate::auth::get_current_user;
use crate::auth::guard::AuthUser;
use crate::models::todolist::{TodoListPageRequest, TodoListPageResponse};
use crate::service::todolist_service::TodolistService;
use crate::utils::response::{ApiResult, ErrorInfo};
use crate::utils::validator::validate_params;
use rocket::form::{Form, FromForm};
use rocket::State;
use rocket_validation::Validated;
use sea_orm::DatabaseConnection;

#[get("/info?<todolist_page_request..>", format = "application/json")]
pub async fn get_todolist(
    auth_user: AuthUser,
    db: &State<DatabaseConnection>,
    todolist_page_request: Validated<TodoListPageRequest>,
) -> ApiResult<TodoListPageResponse, ErrorInfo<()>> {
    debug!("get todolist: {:?}", todolist_page_request);
    let user_info = get_current_user(auth_user, db.inner()).await?;
    TodolistService::get(db.inner(), user_info).await
}
