use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForCreate {
    pub device_name: String,
    pub device_tower: String,
    pub device_room: String,
    pub device_state: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForCreateWrapper {
    pub device: DeviceForCreate,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForSelect {
    pub id: i64,
    pub device_name: String,
    pub device_tower: String,
    pub device_room: String,
    pub device_state: String,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForSelectWrapper {
    pub devices: Vec<DeviceForSelect>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForUpdate {
    pub id: i64,
    pub device_state: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForUpdateWrapper {
    pub device: DeviceForUpdate,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForSelectOne {
    pub id: i64,
    pub device_name: String,
    pub device_tower: String,
    pub device_room: String,
    pub device_state: String,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForSelectOneWrapper {
    pub device: DeviceForSelectOne,
}
