use crate::auth::guard::AuthUser;
use crate::utils::response::{ApiResult, ErrorInfo};
use rocket::serde::json::Json;

#[get("/hello")]
pub fn test_hello(auth_user: AuthUser) -> ApiResult<String, ErrorInfo<()>> {
    Ok(Json(format!("Hello {}", auth_user.user_id)))
}
