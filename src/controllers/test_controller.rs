use crate::auth::guard::AuthUser;
use crate::db::prelude::User;
use crate::utils::response::{ApiError, ApiResult, ErrorInfo};
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{DatabaseConnection, EntityTrait};

#[get("/hello")]
pub async fn test_hello(
    auth_user: AuthUser,
    db: &State<DatabaseConnection>,
) -> ApiResult<String, ErrorInfo<()>> {
    let user_info = User::find().one(db.inner()).await.map_err(|err| {
        ApiError::create_error_info(
            ErrorInfo::DATABASE_ERROR_CODE,
            format!("Database error: {}", err).as_str(),
        )
    })?;
    Ok(Json(format!(
        "Hello {} and {}",
        auth_user.user_id,
        user_info.unwrap().username
    )))
}
