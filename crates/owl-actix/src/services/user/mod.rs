use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

pub mod users;
pub mod users_info;

#[derive(Deserialize, Serialize, Clone, Debug, FromRow)]
pub struct UserId {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccountRegisterInfoWrapper {
    pub account: AccountRegisterInfo,
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
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
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
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OneAccountInfoWrapper {
    pub account: OneAccountInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MemberInfo {
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub gender: String,
    pub age: i64,
    pub user_id: i64,
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
    pub age: i32,
    pub gender: String,
    pub date_enrolled: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AllMembersInfoWrapper {
    pub members: Vec<AllMembersInfo>,
}

impl FromRow<'_, MySqlRow> for AllMembersInfo {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            firstname: row.try_get(1)?,
            middlename: row.try_get(2)?,
            lastname: row.try_get(3)?,
            age: row.try_get(4)?,
            gender: row.try_get(5)?,
            date_enrolled: row.try_get(6)?,
        })
    }
}

impl FromRow<'_, MySqlRow> for OneAccountInfo {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            firstname: row.try_get(0)?,
            middlename: row.try_get(1)?,
            lastname: row.try_get(2)?,
            email_address: row.try_get(3)?,
            gender: row.try_get(4)?,
            recent_address: row.try_get(5)?,
            civil_status: row.try_get(6)?,
            occupation: row.try_get(7)?,
            tower: row.try_get(8)?,
            room: row.try_get(9)?,
            package: row.try_get(10)?,
            date_enrolled: row.try_get(11)?,
        })
    }
}

impl FromRow<'_, MySqlRow> for AccountsInfo {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            firstname: row.try_get(1)?,
            middlename: row.try_get(2)?,
            lastname: row.try_get(3)?,
            email_address: row.try_get(4)?,
            gender: row.try_get(5)?,
            tower: row.try_get(6)?,
            room: row.try_get(7)?,
            package: row.try_get(8)?,
            date_enrolled: row.try_get(9)?,
            status: row.try_get(10)?,
        })
    }
}
