use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterByEmailReq {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResult {}
