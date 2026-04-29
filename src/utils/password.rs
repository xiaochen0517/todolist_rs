use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{Error, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

/// 使用 Argon2 哈希密码
pub fn hash_password_argon2(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

/// 验证 Argon2 密码
pub fn verify_password_argon2(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_argon2() {
        let password = "todolist";
        let hash = hash_password_argon2(password).unwrap();
        println!("hash: {}", hash);
        assert_eq!(true, verify_password_argon2(password, &hash).unwrap());
    }
}
