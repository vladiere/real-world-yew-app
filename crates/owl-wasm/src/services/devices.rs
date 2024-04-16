use crate::{error::Error, types::*};

use super::{request_delete, request_get, request_post};

/// Create device.
pub async fn create_device(body: DeviceForCreateWrapper) -> Result<QueryReturnMessage, Error> {
    request_post::<DeviceForCreateWrapper, QueryReturnMessage>(
        "/admin/device/create".to_string(),
        body,
    )
    .await
}

/// Read all devices.
pub async fn select_devices() -> Result<DeviceForSelectWrapper, Error> {
    request_get::<DeviceForSelectWrapper>("/admin/device/all".to_string()).await
}

/// Update device.
pub async fn update_device(body: DeviceForUpdateWrapper) -> Result<QueryReturnMessage, Error> {
    request_post::<DeviceForUpdateWrapper, QueryReturnMessage>(
        "/admin/device/update".to_string(),
        body,
    )
    .await
}

/// Delete device.
pub async fn delete_device(id: i64) -> Result<QueryReturnMessage, Error> {
    request_delete::<QueryReturnMessage>(format!("/admin/device/delete/{}", id)).await
}

/// Get one device.
pub async fn select_device(id: i64) -> Result<DeviceForSelectOneWrapper, Error> {
    request_get::<DeviceForSelectOneWrapper>(format!("/admin/device/one/{}", id)).await
}
