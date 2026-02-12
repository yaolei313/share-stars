use regex::Regex;

pub fn validate_email(email: &str) -> bool {
    let Ok(re) = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$") else {
        return false;
    };
    re.is_match(email)
}

pub fn mask_email(email: &str) -> String {
    let Some((username, domain)) = email.split_once('@') else {
        // 如果没有找到 '@'，则返回原始字符串
        return email.to_string();
    };
    let len = username.len();
    if len == 1 {
        return format!("*@{}", domain);
    } else if len == 2 {
        return format!("{}*@{}", &username[0..1], domain);
    }

    format!(
        "{}{}{}@{}",
        &username[0..1],
        "*".repeat(len - 2),
        &username[len - 1..],
        domain
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_email() {
        assert_eq!(mask_email("foo@bar"), "f*o@bar");
        assert_eq!(mask_email("a@b.com"), "*@b.com");
        assert_eq!(mask_email("summer@gmail.com"), "s****r@gmail.com");
    }
}
