use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::{Json, serde_json};
use rocket::serde::{Deserialize, Serialize};
use rocket::{Request, Response};
use rocket_validation::{CachedValidationErrors, Error};
use std::io::Cursor;
use std::sync::Mutex;
use validator::ValidationErrors;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T> {
    pub status: Status,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T) -> ApiResponse<T> {
        ApiResponse {
            status: Status::Ok,
            msg: None,
            data: Some(data),
        }
    }

    pub fn error(status: Status, msg: &str) -> Self {
        ApiResponse {
            status,
            msg: Some(msg.to_string()),
            data: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub status: Status,
    pub msg: String,
}

impl ApiError {
    pub fn new(status: Status, msg: String) -> ApiError {
        ApiError { status, msg }
    }

    pub fn internal_server_error_with_msg(msg: &str) -> ApiError {
        ApiError::new(Status::InternalServerError, msg.to_string())
    }

    pub fn internal_server_error() -> ApiError {
        ApiError::internal_server_error_with_msg("Internal Server Error")
    }

    pub fn validation_error(msg: &String) -> ApiError {
        ApiError::new(Status::BadRequest, msg.clone())
    }
    pub fn internal_error(msg: &String) -> ApiError {
        ApiError::new(Status::BadRequest, msg.clone())
    }
}

pub type ApiResult<T> = Result<Json<ApiResponse<T>>, Json<ApiError>>;

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let response = ApiResponse::<()>::error(self.status, &self.msg);
        let json = serde_json::to_string(&response).unwrap_or_else(|_| {
            r#"{"status":500,"msg":"JSON serialization failed","data":null}"#.to_string()
        });
        Response::build()
            .header(rocket::http::ContentType::JSON)
            .status(Status::new(200))
            .sized_body(Some(json.len()), Cursor::new(json))
            .ok()
    }
}

/// 常见的错误类型转换
impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::internal_error(&err.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::validation_error(&err.to_string())
    }
}

impl From<String> for ApiError {
    fn from(err: String) -> Self {
        ApiError::internal_error(&err)
    }
}

#[catch(422)]
pub fn handle_unprocessable_entity<'a>(req: &'a Request) -> Json<ApiResponse<ValidationErrors>> {
    let validation_message = req.local_cache(|| CachedValidationErrors(None)).0.clone();
    Json(ApiResponse {
        status: Status::UnprocessableEntity,
        msg: Some("Unprocessable Entity".to_string()),
        data: validation_message,
    })
}
