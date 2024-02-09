use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdminRegisterInfo {
    pub firstname: String,
    pub lastname: String,
    pub middlename: String,
    pub tower: String,
    pub occupation: String,
    pub position: String,
    pub email_address: String,
    pub contact_number: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdminRegisterInfoWrapper {
    pub admin: AdminRegisterInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterInfo {
    pub client_number: String,
    pub firstname: String,
    pub lastname: String,
    pub middlename: String,
    pub tower: String,
    pub unit: String,
    pub occupation: String,
    pub position: String,
    pub email_address: String,
    pub contact_number: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterInfoWrapper {
    pub user: UserRegisterInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdminInfo {
    pub access_token: String,
    pub refresh_token: String,
}

impl AdminInfo {
    pub fn is_authenticated(&self) -> bool {
        !&self.access_token.is_empty()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdminInfoWrapper {
    pub admin: AdminInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdminUpdateInfo {
    pub firstname: String,
    pub lastname: String,
    pub middlename: String,
    pub tower: String,
    pub occupation: String,
    pub position: String,
    pub email_address: String,
    pub contact_number: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdminUpdateInfoWrapper {
    pub admin: AdminUpdateInfo,
}
