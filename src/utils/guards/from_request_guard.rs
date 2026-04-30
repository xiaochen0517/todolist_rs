use rocket::form;
use rocket::form::{DataField, Form, FromForm, ValueField};
use rocket::http::{RawStr, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket_validation::CachedValidationErrors;
use validator::Validate;

pub struct ValidatedQuery<T>(pub T);

impl<T> ValidatedQuery<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[rocket::async_trait]
impl<'r, T: Validate + FromForm<'r>> FromForm<'r> for ValidatedQuery<T> {
    type Context = T::Context;

    #[inline]
    fn init(opts: form::Options) -> Self::Context {
        T::init(opts)
    }

    #[inline]
    fn push_value(ctxt: &mut Self::Context, field: ValueField<'r>) {
        T::push_value(ctxt, field)
    }

    #[inline]
    async fn push_data(ctxt: &mut Self::Context, field: DataField<'r, '_>) {
        T::push_data(ctxt, field).await
    }

    fn finalize(this: Self::Context) -> form::Result<'r, Self> {
        match T::finalize(this) {
            Err(err) => Err(err),
            Ok(data) => match data.validate() {
                Ok(_) => Ok(ValidatedQuery(data)),
                Err(err) => Err(err
                    .into_errors()
                    .into_iter()
                    .map(|e| form::Error {
                        name: Some(e.0.into()),
                        kind: form::error::ErrorKind::Validation(std::borrow::Cow::Borrowed(e.0)),
                        value: None,
                        entity: form::error::Entity::Value,
                    })
                    .collect::<Vec<_>>()
                    .into()),
            },
        }
    }
}

/// 由于 FromForm 无法直接对接 Validated 返回验证结果， 所以使用 FromRequest 的 from_request 函数加 FromForm 的参数解析功能。
///
/// ```rust
/// #[derive(Debug, Clone, Serialize, Deserialize, FromForm, Validate)]
/// struct ReqData {
///     #[validate(length(min = 3))]
///     name: String,
/// }
///
/// #[get("/hello")]
/// pub async fn hello(params: ValidatedQuery<ReqData>) -> String {
///     format!("Hello, {}!", params.into_inner().name)
/// }
/// ```
///
#[rocket::async_trait]
impl<'r, T> FromRequest<'r> for ValidatedQuery<T>
where
    T: for<'a> rocket::form::FromForm<'a> + Validate + Send + Sync + 'static,
{
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // 利用 Rocket 的 query 解析
        let query_str = req
            .uri()
            .query()
            .map(|q| q.as_str().to_owned())
            .unwrap_or_default();

        match Form::<T>::parse_encoded(RawStr::new(Box::leak(query_str.into_boxed_str()))) {
            Ok(data) => {
                match data.validate() {
                    Ok(_) => Outcome::Success(ValidatedQuery(data)),
                    Err(err) => {
                        // 写入 local_cache，与 POST JSON 422 路径完全一致！
                        req.local_cache(|| CachedValidationErrors(Some(err)));
                        Outcome::Error((Status::UnprocessableEntity, ()))
                    }
                }
            }
            Err(_) => Outcome::Error((Status::UnprocessableEntity, ())),
        }
    }
}
