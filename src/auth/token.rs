use crate::auth::Claims;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

// ⚠️ 生产环境应该从环境变量读取！
const SECRET: &[u8] = b"your-super-secret-key-change-this-in-production";

/// 生成 JWT Token
pub fn generate_token(user_id: String) -> Result<String, String> {
    let claims = Claims::new(user_id);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|e| format!("Failed to generate token: {}", e))
}

/// 验证并解析 JWT Token
pub fn verify_token(token: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Failed to verify token: {}", e))
}
