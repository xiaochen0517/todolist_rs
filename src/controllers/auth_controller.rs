use crate::models::auth::{LoginRequest, LoginResponse};
use crate::service::auth_service::AuthService;
use crate::utils::response::{ApiResult, ErrorInfo};
use rocket::State;
use rocket::serde::json::Json;
use rocket_validation::Validated;
use sea_orm::DatabaseConnection;

/// 用户登录接口
#[post("/login", format = "json", data = "<login_request>")]
pub async fn login(
    db: &State<DatabaseConnection>,
    login_request: Validated<Json<LoginRequest>>,
) -> ApiResult<LoginResponse, ErrorInfo<()>> {
    info!("login request: {:?}", login_request);
    AuthService::login(db.inner(), login_request.into_deep_inner()).await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::{Client, LocalResponse};

    #[derive(Debug, Clone)]
    enum ReqType {
        POST,
        #[warn(dead_code)]
        GET,
    }

    // ✅ 测试 Fixture
    struct TestContext {
        client: Client,
    }

    impl<'a> TestContext {
        fn new() -> Self {
            TestContext {
                client: Client::tracked(rocket()).expect("valid rocket instance"),
            }
        }

        // ✅ 方法接收 &mut self，返回 LocalResponse
        fn request(
            &'a mut self,
            req_type: ReqType,
            url: &'a str,
            login_request: &LoginRequest,
        ) -> LocalResponse<'a> {
            let client = match req_type {
                ReqType::GET => self.client.get(url),
                ReqType::POST => self.client.post(url),
            };
            client.json(login_request).dispatch()
        }
    }

    #[test]
    fn test_login_success() {
        let login_request = LoginRequest {
            username: "admin".to_string(),
            password: "todolist".to_string(),
        };
        let mut context = TestContext::new();
        let response = context.request(ReqType::POST, "/api/auth/login", &login_request);
        // 验证 HTTP 状态码 / Verify HTTP status code
        assert_eq!(response.status(), Status::Ok);
        // 验证响应内容 / Verify response content
        let api_response = response
            .into_json::<LoginResponse>()
            .expect("response should be valid JSON");
        assert_eq!(api_response.token.is_empty(), false);
    }

    #[test]
    fn test_login_validation_error() {
        // 用户名太短 / Username too short
        let login_request = LoginRequest {
            username: "".to_string(),
            password: "todolist".to_string(),
        };
        let mut context = TestContext::new();
        let response = context.request(ReqType::POST, "/api/auth/login", &login_request);
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }
}
