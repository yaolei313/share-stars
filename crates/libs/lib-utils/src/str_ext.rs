pub fn snake_to_pascal_case(s: &str) -> String {
    s.split('_') // 1. 按 '_' 分割字符串
        .map(|word| {
            // 2. 处理每个单词
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(), // 空单词处理
                Some(first_char) => {
                    first_char.to_ascii_uppercase().to_string()
                        + chars.as_str().to_ascii_lowercase().as_str()
                }
            }
        })
        .collect() // 3. 将处理后的单词收集成一个 String
}

pub fn snake_to_camel_case(s: &str) -> String {
    let pascal_case = snake_to_pascal_case(s);
    let mut chars = pascal_case.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_ascii_lowercase().to_string() + chars.as_str(),
    }
}
