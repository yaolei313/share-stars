use crate::db::models::Device;
use crate::db::repositories::{DeviceRepository, PgDeviceRepository};
use crate::db::RepositoryState;
use crate::db::SqlxResult;
use std::sync::Arc;

pub struct DeviceDbService {
    device_repo: Arc<PgDeviceRepository>,
}

impl DeviceDbService {
    pub fn new(repository_state: Arc<RepositoryState>) -> Self {
        DeviceDbService {
            device_repo: repository_state.device_repo.clone(),
        }
    }

    pub async fn query_device(&self, device_id: &str) -> SqlxResult<Option<Device>> {
        self.device_repo.find_by_device_id(device_id).await
    }

    pub async fn save_device(&self, _device: Device) {
        //self.device_repo.find_by_user_id_device_fingerprint();
        todo!()
    }
}
