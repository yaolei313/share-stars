use crate::config::JwtSetting;
use lib_utils::JwtDelegate;

pub struct DeviceService {
    jwt_delegate: JwtDelegate,
}

impl DeviceService {
    pub fn new(device: &JwtSetting) -> anyhow::Result<Self> {
        let jwt_delegate = JwtDelegate::new(&device.keys)?;
        Ok(Self { jwt_delegate })
    }
}
