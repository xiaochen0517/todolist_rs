use validator::Validate;

/// 通用验证函数 / Generic validation function
/// 将验证错误转换为可读的消息字符串
pub fn validate_and_collect_errors<T: Validate>(item: &T) -> Result<(), String> {
    item.validate().map_err(|errors| {
        errors
            .field_errors()
            .iter()
            .map(|(field, field_errors)| {
                // 收集该字段的所有错误消息
                let msgs = field_errors
                    .iter()
                    .map(|err| {
                        err.message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| format!("{}验证失败", field))
                    })
                    .collect::<Vec<_>>()
                    .join("; ");

                format!("{}: {}", field, msgs)
            })
            .collect::<Vec<_>>()
            .join("; ")
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // 占位符测试 / Placeholder test
        assert!(true);
    }
}

