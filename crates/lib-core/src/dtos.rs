use core::str;
use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::*;

#[derive(Debug,Deserialize,Serialize,Validate)]
pub struct RegisterUserDto {


}