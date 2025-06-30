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

// const re = ;

pub fn is_test_phone_number(phone_number: &str) -> bool {
    // https://zh.wikipedia.org/wiki/%E4%B8%AD%E5%9B%BD%E5%A4%A7%E9%99%86%E7%A7%BB%E5%8A%A8%E7%BB%88%E7%AB%AF%E9%80%9A%E4%BF%A1%E5%8F%B7%E7%A0%81#cite_note-8
    // 暂时限制是+86122 1987 ****格式
    let re = get_test_phone_regex();
    re.is_match(phone_number)
}

fn get_test_phone_regex() -> &'static Regex {
    static ITEM: OnceLock<Regex> = OnceLock::new();
    ITEM.get_or_init(|| Regex::new(r"^\+861221987\d{4}$").expect("invalid regex"))
}

#[cfg(test)]
mod tests {
    use crate::is_test_phone_number;

    #[test]
    fn test1() {
        let s1 = "+8612219871234";
        let s2 = "+861221987ABCD";

        println!("'{}' 匹配吗？ {}", s1, is_test_phone_number(s1)); // true
        println!("'{}' 匹配吗？ {}", s2, is_test_phone_number(s2)); // false
    }
}
