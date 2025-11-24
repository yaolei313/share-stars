use lib_utils::JwtDelegate;

pub struct DeviceService {
    jwt_delegate: JwtDelegate,
}

impl DeviceService {
    pub fn new() -> anyhow::Result<Self> {
        todo!()
    }
}
