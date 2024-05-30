use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountRegisterInfo {
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub email_address: String,
    pub gender: String,
    pub recent_address: String,
    pub civil_status: String,
    pub occupation: String,
    pub tower: String,
    pub room: String,
    pub package: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountRegisterInfoWrapper {
    pub account: AccountRegisterInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AllAdminInfo {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub middlename: String,
    pub email_address: String,
    pub username: String,
    pub date_enrolled: String,
    pub status: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AllAdminInfoWrapper {
    pub admins: Vec<AllAdminInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OneAdminInfo {
    pub firstname: String,
    pub lastname: String,
    pub middlename: String,
    pub email_address: String,
    pub gender: String,
    pub recent_address: String,
    pub civil_status: String,
    pub occupation: String,
    pub username: String,
    pub date_enrolled: String,
    pub status: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OneAdminInfoWrapper {
    pub admin: OneAdminInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOneAdminInfo {
    pub admin_id: i64,
    pub email_address: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOneAdminInfoWrapper {
    pub admin: UpdateOneAdminInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccountsInfo {
    pub id: i64,
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub email_address: String,
    pub gender: String,
    pub tower: String,
    pub room: String,
    pub package: String,
    pub date_enrolled: String,
    pub status: String,
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountsInfoWrapper {
    pub accounts: Vec<AccountsInfo>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OneAccountInfo {
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub email_address: String,
    pub gender: String,
    pub recent_address: String,
    pub civil_status: String,
    pub occupation: String,
    pub tower: String,
    pub room: String,
    pub package: String,
    pub date_enrolled: String,
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OneAccountInfoWrapper {
    pub account: OneAccountInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MemberInfo {
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub gender: String,
    pub age: i64,
    pub user_id: i64,
    pub member_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MemberInfoWrapper {
    pub member: MemberInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AllMembersInfo {
    pub id: i64,
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub age: i64,
    pub gender: String,
    pub date_enrolled: String,
    pub member_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AllMembersInfoWrapper {
    pub members: Vec<AllMembersInfo>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FileDetails {
    pub name: String,
    pub file_type: String,
    pub data: Vec<u8>,
}
