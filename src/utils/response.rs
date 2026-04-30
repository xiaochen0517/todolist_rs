use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorInfo<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl ErrorInfo<()> {
    pub(crate) const DEFAULT_ERROR_CODE: i32 = 1000;

    /// 请求体 JSON 序列化检查错误
    pub(crate) const REQUEST_BODY_PARAMS_ERROR_CODE: i32 = 4000;

    /// 用户或密码错误
    pub(crate) const USERNAME_OR_PASSWORD_ERROR_CODE: i32 = 4100;

    /// 登录状态验证失败
    pub(crate) const AUTHORIZATION_ERROR_CODE: i32 = 4101;

    /// TOKEN 生成失败
    pub(crate) const GENERATE_TOKEN_ERROR_CODE: i32 = 5100;

    /// 数据库查询失败
    pub(crate) const DATABASE_ERROR_CODE: i32 = 6000;

    /// 未发现用户
    pub(crate) const USER_NOT_FOUND_ERROR_CODE: i32 = 6100;

    pub fn new(code: i32, message: &str) -> ErrorInfo<()> {
        ErrorInfo {
            code,
            message: message.to_string(),
            data: None,
        }
    }
}

impl<T> ErrorInfo<T> {
    pub fn new_with_data(code: i32, message: &str, data: T) -> ErrorInfo<T> {
        ErrorInfo {
            code,
            message: message.to_string(),
            data: Some(data),
        }
    }
}

#[derive(Responder, Debug, Clone)]
pub enum ApiError<T> {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<T>),
    #[response(status = 401, content_type = "json")]
    Unauthorized(Json<T>),
    #[response(status = 404, content_type = "json")]
    NotFound(Json<T>),

    #[response(status = 500, content_type = "json")]
    InternalError(Json<T>),
}

impl ApiError<ErrorInfo<()>> {
    pub fn create_error_info(code: i32, message: &str) -> ApiError<ErrorInfo<()>> {
        ApiError::BadRequest(Json(ErrorInfo::new(code, message)))
    }

    pub fn create_unauthorized(code: i32, message: &str) -> ApiError<ErrorInfo<()>> {
        ApiError::Unauthorized(Json(ErrorInfo::new(code, message)))
    }

    pub fn create_internal_error(code: i32, message: &str) -> ApiError<ErrorInfo<()>> {
        ApiError::InternalError(Json(ErrorInfo::new(code, message)))
    }
}

pub type ApiResult<T, E> = Result<Json<T>, ApiError<E>>;
