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

pub fn mask_unicode_string(input: &str, prefix_len: usize, mask_len: usize) -> String {
    // 1. 获取输入字符串的字符迭代器
    let mut chars = input.chars();

    // 2. 获取前缀部分
    let prefix: String = chars.by_ref().take(prefix_len).collect();

    // 3. 确定剩余部分
    let remaining_chars_count = chars.clone().count(); // 剩余字符数量

    // 4. 确定实际掩码长度
    let actual_mask_len = mask_len.min(remaining_chars_count);

    // 5. 跳过被掩码的部分
    let remaining_after_mask: String = chars
        .skip(actual_mask_len) // 跳过需要掩码的字符
        .collect(); // 收集剩余的字符

    // 6. 组合最终脱敏的字符串
    format!(
        "{}{}{}",
        prefix,
        "*".repeat(actual_mask_len),
        remaining_after_mask
    )
}

pub fn mask_ascii_string(input: &str, prefix_len: usize, mask_len: usize) -> String {
    let input_len = input.len(); // 这里的长度 (字节数) == 字符数

    // 1. 处理边界情况：如果输入字符串长度不足
    if input_len <= prefix_len {
        return input.to_string();
    }

    // 2. 确定要显示的实际前缀（基于字节切片）
    // 这种切片方式在确认是 ASCII 字符串时是安全的且高效的
    let prefix = &input[..prefix_len];

    // 3. 确定要隐藏的部分的实际长度
    let remaining_len = input_len - prefix_len;

    // 4. 确定要使用的星号数量：
    let actual_mask_len = mask_len.min(remaining_len);

    // 5. 确定星号后显示的后缀的起始索引
    let suffix_start_index = prefix_len + actual_mask_len;

    // 6. 获取后缀部分
    // 注意：如果实际掩码长度小于剩余长度，后面仍有部分需要显示
    let suffix = &input[suffix_start_index..];

    // 7. 组合最终脱敏的字符串
    format!("{}{}{}", prefix, "*".repeat(actual_mask_len), suffix)
}
