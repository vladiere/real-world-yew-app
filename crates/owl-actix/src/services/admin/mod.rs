use serde::{Deserialize, Serialize};

pub mod admin_info;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAdminInfo {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub middlename: String,
    pub tower: String,
    pub occupation: String,
    pub position: String,
    pub email_address: String,
    pub contact_number: String,
    pub username: String,
    pub role_user: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAdminInfoWrapper {
    pub admin: CurrentAdminInfo,
}
