use rand::{Rng, RngCore};

pub fn rand_verify_code() -> String {
    let mut rng = rand::rng();
    let random_number: u32 = rng.random_range(100_000..=999_999);
    random_number.to_string()
}

pub fn rand_salt_hex_string() -> String {
    let mut rng = rand::rng();
    let mut data_u8 = [0u8; 8];
    rng.fill_bytes(&mut data_u8);
    bytes_to_hex_string(&data_u8)
}

pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let mut hex_string = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        // format! 宏将 u8 格式化为两位十六进制数，不足两位前补零
        hex_string.push_str(&format!("{:02x}", byte));
    }
    hex_string
}
