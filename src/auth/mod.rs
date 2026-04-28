pub mod guard;
pub mod token;

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

// JWT 中的用户声明（Claims）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // 用户 ID
    pub exp: i64,    // 过期时间（Unix 时间戳）
    pub iat: i64,    // 签发时间
}

impl Claims {
    /// 创建新的 Claims，有效期 24 小时
    pub fn new(user_id: String) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::hours(24);

        Claims {
            sub: user_id,
            exp: expiry.timestamp(),
            iat: now.timestamp(),
        }
    }
}
