#[derive(Debug)]
pub enum ErrorCode {
    ServerError,
    EmailExist,
    InvalidCredentials,
    EmptyPassword,
    ExceedMaxPasswordLength,
    InvalidToken,
}
