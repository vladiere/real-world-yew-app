use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

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
