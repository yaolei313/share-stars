use std::collections::HashMap;

pub fn parse_query_string_to_map(query_string: &str) -> HashMap<String, String> {
    query_string
        .split('&')
        .filter_map(|pair| {
            // 确保我们只处理非空的对
            if pair.is_empty() {
                return None;
            }
            let mut parts = pair.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                // 对于只有 key 没有 value 的情况，value 设为空字符串
                (Some(key), None) => Some((key.to_string(), String::new())),
                _ => None,
            }
        })
        .collect()
}
