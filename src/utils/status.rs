use rocket::http::Status;

/// 业务错误类型定义 / Business error type
#[derive(Debug, Clone)]
pub enum BizError {
    /// 验证失败 / Validation failed
    ValidationError(String),
    /// 未授权 / Unauthorized
    Unauthorized(String),
    /// 禁止访问 / Forbidden
    Forbidden(String),
    /// 资源不存在 / Not found
    NotFound(String),
    /// 业务逻辑错误 / Business logic error
    BusinessLogicError(String),
    /// 服务器内部错误 / Internal server error
    InternalError(String),
}

impl BizError {
    /// 获取 HTTP 状态码 / Get HTTP status code
    pub fn http_status(&self) -> Status {
        match self {
            BizError::ValidationError(_) => Status::BadRequest,
            BizError::Unauthorized(_) => Status::Unauthorized,
            BizError::Forbidden(_) => Status::Forbidden,
            BizError::NotFound(_) => Status::NotFound,
            BizError::BusinessLogicError(_) => Status::BadRequest,
            BizError::InternalError(_) => Status::InternalServerError,
        }
    }

    /// 获取业务错误码 / Get business error code
    pub fn code(&self) -> u16 {
        match self {
            BizError::ValidationError(_) => 4001,
            BizError::Unauthorized(_) => 4011,
            BizError::Forbidden(_) => 4031,
            BizError::NotFound(_) => 4041,
            BizError::BusinessLogicError(_) => 4002,
            BizError::InternalError(_) => 5001,
        }
    }

    /// 获取错误消息 / Get error message
    pub fn message(&self) -> &str {
        match self {
            BizError::ValidationError(msg) => msg,
            BizError::Unauthorized(msg) => msg,
            BizError::Forbidden(msg) => msg,
            BizError::NotFound(msg) => msg,
            BizError::BusinessLogicError(msg) => msg,
            BizError::InternalError(msg) => msg,
        }
    }
}
