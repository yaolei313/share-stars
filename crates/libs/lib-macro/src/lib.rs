pub trait ToResponse {
    fn code() -> i32;

    fn message() -> String;
}
