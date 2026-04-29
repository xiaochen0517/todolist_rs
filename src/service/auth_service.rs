use crate::auth::token::generate_token;
use crate::db::prelude::User;
use crate::db::user;
use crate::models::auth::{LoginRequest, LoginResponse};
use crate::sea_orm::QueryFilter;
use crate::utils::password;
use crate::utils::response::{ApiError, ApiResult, ErrorInfo};
use rocket::serde::json::Json;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};

pub struct AuthService {}

impl AuthService {
    pub async fn login(
        db: &DatabaseConnection,
        login_request: LoginRequest,
    ) -> ApiResult<LoginResponse, ErrorInfo<()>> {
        let user_info = User::find()
            .filter(user::Column::Username.eq(login_request.username.as_str()))
            .one(db)
            .await
            .map_err(|err| {
                ApiError::create_error_info(
                    ErrorInfo::DATABASE_ERROR_CODE,
                    format!("Database error: {}", err).as_str(),
                )
            })?;
        let user_info = match user_info {
            Some(user_info) => user_info,
            None => {
                return Err(ApiError::create_unauthorized(
                    ErrorInfo::AUTHORIZATION_ERROR_CODE,
                    "Username or password error",
                ));
            }
        };
        let is_valid = password::verify_password_argon2(
            login_request.password.as_str(),
            user_info.password.as_str(),
        )
        .map_err(|_| {
            ApiError::create_unauthorized(
                ErrorInfo::AUTHORIZATION_ERROR_CODE,
                "Username or password error",
            )
        })?;
        if !is_valid {
            return Err(ApiError::create_unauthorized(
                ErrorInfo::AUTHORIZATION_ERROR_CODE,
                "Username or password error",
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
