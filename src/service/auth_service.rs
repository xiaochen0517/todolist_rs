use crate::auth::token::generate_token;
use crate::models::auth::{LoginRequest, LoginResponse};
use crate::utils::response::{ApiError, ApiResult, ErrorInfo};
use rocket::serde::json::Json;

pub struct AuthService {}

impl AuthService {
    pub fn login(login_request: LoginRequest) -> ApiResult<LoginResponse, ErrorInfo<()>> {
        if !login_request.username.eq("masonlee") {
            return Err(ApiError::create_error_info(
                ErrorInfo::USERNAME_OR_PASSWORD_ERROR_CODE,
                format!("Username does not exist: {}", login_request.username).as_str(),
            ));
        }
        // 生成 token
        match generate_token(login_request.username.clone()) {
            Ok(token) => Ok(Json(LoginResponse { token })),
            Err(err) => Err(ApiError::create_error_info(
                ErrorInfo::USERNAME_OR_PASSWORD_ERROR_CODE,
                err.as_str(),
            )),
        }
    }
}
