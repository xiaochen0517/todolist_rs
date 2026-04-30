use crate::utils::response::ErrorInfo;
use rocket::serde::json::Json;
use rocket::Request;
use rocket_validation::CachedValidationErrors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidationError {
    pub param_name: String,
    pub errors: Vec<String>,
}

#[catch(422)]
pub fn handle_unprocessable_entity<'a>(
    req: &'a Request,
) -> Option<Json<ErrorInfo<Vec<ValidationError>>>> {
    let validation_message = req.local_cache(|| CachedValidationErrors(None)).0.clone();
    match validation_message {
        None => Some(Json(ErrorInfo {
            code: ErrorInfo::REQUEST_BODY_PARAMS_ERROR_CODE,
            message: "请求参数验证失败，请检查输入的参数是否正确".to_string(),
            data: None,
        })),
        Some(data) => {
            let validation_errors: Vec<ValidationError> = data
                .field_errors()
                .iter()
                .map(|(param_name, errors)| ValidationError {
                    param_name: param_name.to_string(),
                    errors: errors
                        .iter()
                        .map(|error| {
                            if let Some(message) = &error.message {
                                message.to_string()
                            } else {
                                format!("Validation error on parameter: {}", param_name)
                            }
                        })
                        .collect(),
                })
                .collect();
            Some(Json(ErrorInfo::new_with_data(
                ErrorInfo::REQUEST_BODY_PARAMS_ERROR_CODE,
                "请求参数验证失败，请检查输入的参数是否正确",
                validation_errors,
            )))
        }
    }
}

#[catch(401)]
pub fn handle_unauthorized<'a>(req: &'a Request) -> Json<ErrorInfo<()>> {
    use crate::auth::guard::AuthErrorCache;

    // 从请求本地存储中获取 guard 返回的错误信息
    let auth_error = req.local_cache(|| AuthErrorCache(None));

    if let Some(error_info) = &auth_error.0 {
        return Json(error_info.clone());
    }

    // 如果没有找到缓存的错误信息，返回一个通用的 401 错误
    Json(ErrorInfo::new(
        ErrorInfo::AUTHORIZATION_ERROR_CODE,
        "登录状态验证失败，请提供有效的认证凭证",
    ))
}
