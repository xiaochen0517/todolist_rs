use crate::utils::catcher::ValidationError;
use crate::utils::response::{ApiError, ApiResult, ErrorInfo};
use rocket::serde::json::Json;
use validator::Validate;

pub fn validate_params<T: Validate>(data: &T) -> ApiResult<(), ErrorInfo<Vec<ValidationError>>> {
    data.validate().map_err(|err| {
        let errors: Vec<ValidationError> = err
            .field_errors()
            .iter()
            .map(|(param_name, errors)| ValidationError {
                param_name: param_name.to_string(),
                errors: errors
                    .iter()
                    .map(|e| {
                        e.message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| format!("Validation error on: {}", param_name))
                    })
                    .collect(),
            })
            .collect();
        ApiError::BadRequest(Json(ErrorInfo::new_with_data(
            ErrorInfo::REQUEST_BODY_PARAMS_ERROR_CODE,
            "请求参数验证失败，请检查输入的参数是否正确",
            errors,
        )))
    })?;
    Ok(Json(()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // 占位符测试 / Placeholder test
        assert!(true);
    }
}
