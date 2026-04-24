use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    #[validate(length(min = 1, max = 24, message = "用户名长度必须在 1-24 个字符之间"))]
    pub username: String,
    #[validate(length(min = 8, max = 24, message = "密码长度必须在 8-24 个字符之间"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    pub token: String,
}
