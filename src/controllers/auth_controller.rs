use crate::models::auth::{LoginRequest, LoginResponse};
use crate::utils::response::{ApiResponse, ApiResult};
use rocket::serde::json::Json;
use rocket_validation::Validated;

#[post("/login", format = "json", data = "<login_request>")]
pub fn login(login_request: Validated<Json<LoginRequest>>) -> ApiResult<LoginResponse> {
    info!("login request: {:?}", login_request);
    // 业务逻辑处理 / Business logic processing
    Ok(Json(ApiResponse::new(LoginResponse {
        token: "test token".to_string(),
    })))
}
