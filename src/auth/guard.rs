use crate::auth::token::verify_token;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

/// 认证用户守卫 - 在路由处理函数中使用
pub struct AuthUser {
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // 从 Authorization 头中提取 token
        let headers: Vec<_> = request.headers().get("Authorization").collect();

        if headers.len() != 1 {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let auth_header = headers[0];

        // 期望格式: "Bearer <token>"
        if !auth_header.starts_with("Bearer ") {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let token = &auth_header[7..];

        // 验证 token
        match verify_token(token) {
            Ok(claims) => Outcome::Success(AuthUser {
                user_id: claims.sub,
            }),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
