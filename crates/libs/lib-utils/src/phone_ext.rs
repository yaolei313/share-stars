use phonenumber::Mode;
use regex::Regex;
use std::sync::OnceLock;

pub fn validate_then_format_phone_number(
    phone_number: &str,
) -> Result<String, phonenumber::ParseError> {
    let number = phonenumber::parse(None, phone_number)?;
    let e164_phone = number.format().mode(Mode::E164).to_string();
    Ok(e164_phone)
}

pub fn mask_e164_phone_number(phone_number: &str) -> String {
    let len = phone_number.len();
    if len < 5 {
        return phone_number.to_string();
    }
    if phone_number.starts_with("+") {
        format!("+{}{}", "*".repeat(len - 5), &phone_number[len - 4..])
    } else {
        format!("+{}{}", "*".repeat(len - 4), &phone_number[len - 4..])
    }
}

// const re = ;

pub fn is_test_phone_number(phone_number: &str) -> bool {
    // https://zh.wikipedia.org/wiki/%E4%B8%AD%E5%9B%BD%E5%A4%A7%E9%99%86%E7%A7%BB%E5%8A%A8%E7%BB%88%E7%AB%AF%E9%80%9A%E4%BF%A1%E5%8F%B7%E7%A0%81#cite_note-8
    // 暂时限制是+86122 1987 ****格式
    let re = get_test_phone_regex();
    re.is_match(phone_number)
}

pub fn is_test_email(email: &str) -> bool {
    let re = get_email_regex();
    re.is_match(email)
}

fn get_test_phone_regex() -> &'static Regex {
    static ITEM: OnceLock<Regex> = OnceLock::new();
    ITEM.get_or_init(|| Regex::new(r"^\+861221987\d{4}$").expect("invalid regex"))
}

fn get_email_regex() -> &'static Regex {
    static EMAIL_ITEM: OnceLock<Regex> = OnceLock::new();
    EMAIL_ITEM.get_or_init(|| Regex::new(r"^\w+@app\.yao\.com$").expect("invalid regex"))
}

#[cfg(test)]
mod tests {
    use crate::{is_test_phone_number, mask_e164_phone_number};

    #[test]
    fn test1() {
        let s1 = "+8612219871234";
        let s2 = "+861221987ABCD";

        println!("'{}' 匹配吗？ {}", s1, is_test_phone_number(s1)); // true
        println!("'{}' 匹配吗？ {}", s2, is_test_phone_number(s2)); // false
    }

    #[test]
    fn test2() {
        let phone1 = "+8613800138000";
        let phone2 = "+15551234";
        let phone3 = "1234567"; // 不带 + 号
        let phone4 = "+123"; // 短于 4 位数字

        println!("{} -> {}", phone1, mask_e164_phone_number(phone1));
        println!("{} -> {}", phone2, mask_e164_phone_number(phone2));
        println!("{} -> {}", phone3, mask_e164_phone_number(phone3));
        println!("{} -> {}", phone4, mask_e164_phone_number(phone4));
    }
}
