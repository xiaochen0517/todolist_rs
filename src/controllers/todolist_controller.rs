use crate::auth::get_current_user;
use crate::auth::guard::AuthUser;
use crate::models::todolist::{TodoListPageRequest, TodoListPageResponse};
use crate::service::todolist_service::TodolistService;
use crate::utils::guards::from_request_guard::ValidatedQuery;
use crate::utils::response::{ApiResult, ErrorInfo};
use rocket::State;
use sea_orm::DatabaseConnection;

#[get("/info", format = "json")]
pub async fn get_todolist(
    auth_user: AuthUser,
    db: &State<DatabaseConnection>,
    todolist_page_request: ValidatedQuery<TodoListPageRequest>,
) -> ApiResult<TodoListPageResponse, ErrorInfo<()>> {
    let request_data = todolist_page_request.into_inner();
    debug!("get todolist: {:?}", request_data);
    let user_info = get_current_user(auth_user, db.inner()).await?;
    TodolistService::get(db.inner(), user_info, &request_data).await
}
