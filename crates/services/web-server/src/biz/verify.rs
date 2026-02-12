mod code_manager;
mod verify_manager;

pub use code_manager::*;
pub use verify_manager::*;

use lib_macro_derive::BindCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, BindCode, Copy, Clone)]
pub enum VerifyScenario {
    #[code(1)]
    Login,
    #[code(2)]
    BindPhone,
    #[code(3)]
    ResetPwd,
    #[code(4)]
    Mfa,
}
