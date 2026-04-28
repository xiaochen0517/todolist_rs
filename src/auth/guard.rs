use crate::auth::token::verify_token;
use crate::utils::response::ErrorInfo;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

/// 认证用户守卫 - 在路由处理函数中使用
pub struct AuthUser {
    pub user_id: String,
}

/// 用于在请求本地存储中存储认证错误信息
pub struct AuthErrorCache(pub Option<ErrorInfo<()>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ErrorInfo<()>;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // 从 Authorization 头中提取 token
        let headers: Vec<_> = request.headers().get("Authorization").collect();

        if headers.len() != 1 {
            let error_info = ErrorInfo::new(
                ErrorInfo::AUTHORIZATION_ERROR_CODE,
                "Authorization header is missing or invalid",
            );
            // 将错误信息存储到请求本地存储中，供 catcher 使用
            request.local_cache(|| AuthErrorCache(Some(error_info.clone())));
            return Outcome::Error((Status::Unauthorized, error_info));
        }

        let auth_header = headers[0];

        // 期望格式: "Bearer <token>"
        if !auth_header.starts_with("Bearer ") {
            let error_info = ErrorInfo::new(
                ErrorInfo::AUTHORIZATION_ERROR_CODE,
                "Authorization header is missing or invalid",
            );
            request.local_cache(|| AuthErrorCache(Some(error_info.clone())));
            return Outcome::Error((Status::Unauthorized, error_info));
        }

        let token = &auth_header[7..];

        // 验证 token
        match verify_token(token) {
            Ok(claims) => Outcome::Success(AuthUser {
                user_id: claims.sub,
            }),
            Err(_) => {
                let error_info = ErrorInfo::new(
                    ErrorInfo::AUTHORIZATION_ERROR_CODE,
                    "Authorization header is missing or invalid",
                );
                request.local_cache(|| AuthErrorCache(Some(error_info.clone())));
                Outcome::Error((Status::Unauthorized, error_info))
            }
        }
    }
}
