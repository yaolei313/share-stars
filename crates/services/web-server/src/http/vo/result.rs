use crate::http::vo::RespVo;
use axum::Json;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub struct BizResultCode(i32, &'static str);

#[macro_export]
macro_rules! biz_result_codes {
    (
        $(
            $(#[$docs:meta])*
            $konst:ident($code:expr, $message:expr);
        )+
    ) => {
        impl BizResultCode {
        $(
            $(#[$docs])*
            pub const $konst: BizResultCode = BizResultCode($code, $message);
        )+

        }
    }
}

biz_result_codes! {
    /// success
    SUCCESS(200,"");
    INVALID_ARGUMENT(201,"");
    BAD_REQUEST(400,"");
    UNAUTHORIZED(401,"Unauthorized");
}

impl BizResultCode {
    pub fn code(&self) -> i32 {
        self.0
    }
    pub fn message(&self) -> &'static str {
        self.1
    }
}

impl IntoResponse for BizResultCode {
    fn into_response(self) -> Response {
        let vo: RespVo<()> = RespVo {
            code: self.0,
            message: Some(self.1.to_string()),
            data: None,
        };
        Json(vo).into_response()
    }
}
