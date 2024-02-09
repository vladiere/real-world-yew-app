use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccountsInfo {
    pub client_number: String,
    pub name: String,
    pub tower: String,
    pub unit: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccountsInfoWrapper {
    pub accounts: AccountsInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccountRegisterInfo {
    pub client_number: String,
    pub name: String,
    pub tower: String,
    pub unit: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccountRegisterInfoWrapper {
    pub accounts: AccountRegisterInfo,
}
