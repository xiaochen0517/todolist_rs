use crate::models::auth::{LoginRequest, LoginResponse};
use crate::utils::response::{ApiResponse, ApiResult};
use rocket::serde::json::Json;
use rocket_validation::Validated;

#[post("/login", format = "json", data = "<login_request>")]
pub fn login(login_request: Validated<Json<LoginRequest>>) -> ApiResult<LoginResponse> {
    info!("login request: {:?}", login_request);
    // 业务逻辑处理 / Business logic processing
    Ok(Json(ApiResponse::new(LoginResponse {
        token: "test token".to_string(),
    })))
}

#[cfg(test)]
mod test {
    use crate::models::auth::{LoginRequest, LoginResponse};
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use crate::utils::response::ApiResponse;
    use crate::utils::status::ResponseStatus;

    #[test]
    fn test_login() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let login_request_data = LoginRequest {
            username: "testuser".to_string(),
            password: "testpassword".to_string(),
        };
        let response = client
            .post("/api/auth/login")
            .json(&login_request_data)
            .dispatch();

        // 验证 HTTP 状态码
        assert_eq!(response.status(), Status::Ok);

        // 正确的方式：反序列化为 ApiResponse<LoginResponse>
        let api_response = response
            .into_json::<ApiResponse<LoginResponse>>()
            .expect("response should be valid JSON");

        // 验证响应数据
        assert_eq!(api_response.status, ResponseStatus::SUCCESS);
        assert!(api_response.data.is_some());
        assert_eq!(api_response.data.unwrap().token, "test token");
    }
}
