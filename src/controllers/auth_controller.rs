use crate::models::auth::{LoginRequest, LoginResponse};
use crate::service::auth_service::AuthService;
use crate::utils::response::{ApiResult, ErrorInfo};
use rocket::serde::json::Json;
use rocket_validation::Validated;

/// 用户登录接口
#[post("/login", format = "json", data = "<login_request>")]
pub fn login(
    login_request: Validated<Json<LoginRequest>>,
) -> ApiResult<LoginResponse, ErrorInfo<()>> {
    info!("login request: {:?}", login_request);
    AuthService::login(login_request.into_deep_inner())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_login_success() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let login_request = LoginRequest {
            username: "testuser".to_string(),
            password: "testpassword123".to_string(),
        };

        let response = client
            .post("/api/auth/login")
            .json(&login_request)
            .dispatch();

        // 验证 HTTP 状态码 / Verify HTTP status code
        assert_eq!(response.status(), Status::Ok);

        // 验证响应内容 / Verify response content
        let api_response = response
            .into_json::<LoginResponse>()
            .expect("response should be valid JSON");

        assert!(api_response.token.starts_with("token_testuser"));
    }

    #[test]
    fn test_login_validation_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // 用户名太短 / Username too short
        let login_request = LoginRequest {
            username: "".to_string(),
            password: "testpassword123".to_string(),
        };

        let response = client
            .post("/api/auth/login")
            .json(&login_request)
            .dispatch();

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }
}
