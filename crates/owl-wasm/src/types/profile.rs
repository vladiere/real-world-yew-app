use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfo {
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub tower: String,
    pub unit: String,
    pub occupation: String,
    pub position: String,
    pub email_address: String,
    pub contact_number: String,
    pub img_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfoWrapper {
    pub profile: ProfileInfo,
}
