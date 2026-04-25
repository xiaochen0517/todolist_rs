use rocket::serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct ResponseStatus {
    code: u16,
}

impl Default for ResponseStatus {
    fn default() -> Self {
        ResponseStatus { code: 200 }
    }
}

impl ResponseStatus {
    pub const SUCCESS: ResponseStatus = ResponseStatus { code: 200 };
    pub const CLIENT_ERROR: ResponseStatus = ResponseStatus { code: 401 };
    pub const SERVER_FAILURE: ResponseStatus = ResponseStatus { code: 500 };

    fn new(code: u16) -> Self {
        ResponseStatus { code }
    }
}

/// 重写序列化函数
mod serde {
    use super::*;
    use rocket::serde::de::{Error, Unexpected, Visitor};
    use std::fmt;

    impl<'a> Serialize for ResponseStatus {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_u16(self.code)
        }
    }

    struct DeVisitor;

    impl<'de> Visitor<'de> for DeVisitor {
        type Value = ResponseStatus;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "HTTP status code integer in range [100, 600)")
        }

        fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
            if v < 100 || v >= 600 {
                return Err(E::invalid_value(Unexpected::Signed(v), &self));
            }

            Ok(ResponseStatus::new(v as u16))
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            if v < 100 || v >= 600 {
                return Err(E::invalid_value(Unexpected::Unsigned(v), &self));
            }

            Ok(ResponseStatus::new(v as u16))
        }
    }

    impl<'de> Deserialize<'de> for ResponseStatus {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_u16(DeVisitor)
        }
    }
}
