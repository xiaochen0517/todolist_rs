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
        None => None,
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
