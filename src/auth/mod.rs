pub mod guard;
pub mod token;

use crate::auth::guard::AuthUser;
use crate::db::prelude::User;
use crate::db::user;
use crate::utils::response::{ApiError, ErrorInfo};
use chrono::{Duration, Utc};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

// JWT 中的用户声明（Claims）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, // 用户 ID
    pub exp: i64, // 过期时间（Unix 时间戳）
    pub iat: i64, // 签发时间
}

impl Claims {
    /// 创建新的 Claims，有效期 24 小时
    pub fn new(user_id: i32) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::hours(24);

        Claims {
            sub: user_id,
            exp: expiry.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub async fn get_current_user(
    auth_user: AuthUser,
    db: &DatabaseConnection,
) -> Result<user::Model, ApiError<ErrorInfo<()>>> {
    User::find_by_id(auth_user.user_id)
        .one(db)
        .await
        .map_err(|err| {
            ApiError::create_internal_error(
                ErrorInfo::DATABASE_ERROR_CODE,
                format!("Database error: {}", err).as_str(),
            )
        })?
        .ok_or(ApiError::create_internal_error(
            ErrorInfo::USER_NOT_FOUND_ERROR_CODE,
            "User not found",
        ))
}
