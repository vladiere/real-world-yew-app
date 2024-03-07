use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

pub mod admin;
pub mod admin_info;

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentAdminInfo {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub middlename: String,
    pub email_address: String,
    pub username: String,
    pub role_user: String,
    pub token_salt: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAdminInfoWithToken {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub middlename: String,
    pub email_address: String,
    pub username: String,
    pub role_user: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAdminInfoWrapper {
    pub admin: CurrentAdminInfoWithToken,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdminsInfo {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub middlename: String,
    pub email_address: String,
    pub username: String,
    pub date_enrolled: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdminsInfoWrapper {
    pub admins: Vec<AdminsInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdminUpdate {
    pub password: String,
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
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOneAdmin {
    pub admin_id: i64,
    pub email_address: String,
    pub username: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOneAdminWrapper {
    pub admin: UpdateOneAdmin,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OneAdminInfoWrapper {
    pub admin: OneAdminInfo,
}

impl FromRow<'_, MySqlRow> for OneAdminInfo {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            firstname: row.try_get(0)?,
            lastname: row.try_get(1)?,
            middlename: row.try_get(2)?,
            email_address: row.try_get(3)?,
            gender: row.try_get(4)?,
            recent_address: row.try_get(5)?,
            civil_status: row.try_get(6)?,
            occupation: row.try_get(7)?,
            username: row.try_get(8)?,
            date_enrolled: row.try_get(9)?,
            status: row.try_get(10)?,
        })
    }
}

impl FromRow<'_, MySqlRow> for CurrentAdminInfo {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            firstname: row.try_get(1)?,
            lastname: row.try_get(2)?,
            middlename: row.try_get(3)?,
            email_address: row.try_get(4)?,
            username: row.try_get(5)?,
            role_user: row.try_get(6)?,
            token_salt: row.try_get(7)?,
            // Map other fields...
        })
    }
}

impl FromRow<'_, MySqlRow> for AdminsInfo {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            firstname: row.try_get(1)?,
            lastname: row.try_get(2)?,
            middlename: row.try_get(3)?,
            email_address: row.try_get(4)?,
            username: row.try_get(5)?,
            date_enrolled: row.try_get(6)?,
            status: row.try_get(7)?,
        })
    }
}
