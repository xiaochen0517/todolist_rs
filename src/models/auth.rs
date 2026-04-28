use rocket_validation::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, max = 24, message = "用户名长度必须在 1-24 个字符之间"))]
    pub username: String,
    #[validate(length(min = 8, max = 24, message = "密码长度必须在 8-24 个字符之间"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub token: String,
}
