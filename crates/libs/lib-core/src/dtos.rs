use crate::models::*;
use chrono::{DateTime, Utc};
use core::str;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterUserDto {
    #[valdate(length(min = 8, max = 32, message = "name is required"))]
    pub name: String,

    #[valdate(
        length(min = 3, max = 64, message = "email is required"),
        email(message = "email is invalid")
    )]
    pub email: String,

    #[validate(length(min = 6, max = 16, message = "password must at least 6 characters"))]
    pub password: String,

    #[validate(
        length(min = 8, max = 32, message = "password is required"),
        must_match(other = "password", message = "password do not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginUserDto {
    #[valdate(
        length(min = 3, max = 64, message = "email is required"),
        email(message = "email is invalid")
    )]
    pub email: String,

    #[validate(length(min = 6, max = 16, message = "password must at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RequestQueryDto {
    #[valdate(range(min = 1))]
    pub page: Option<usize>,
    #[valdate(range(min = 10, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct FilterUserDto {}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {}
    }

    pub fn filter_users(users: &[User]) -> Vec<Self> {
        users.iter().map(FilterUserDto::filter_user).collect()
    }
}
