pub mod admin;
mod auth;
pub mod extract_token;

// pub use admin::*;
pub use auth::*;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};
use tracing::debug;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Deserialize, sqlx::Type, Serialize, Clone)]
#[sqlx(type_name = "user_type")]
pub enum UserType {
    Admin,
    Super,
    User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthAdmin {
    id: i64,
    username: String,
    password: String,
    token_salt: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleUser {
    role_user: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterAdminBody {
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub tower: String,
    pub occupation: String,
    pub position: String,
    pub email_address: String,
    pub contact_number: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, FromRow)]
pub struct AdminNoPassword {
    pub id: i64,
}

#[derive(Serialize, FromRow)]
pub struct AdminId {
    pub id: i64,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginAdminInfo {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginAdmin {
    pub user: LoginAdminInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseToken {
    pub admin: TokenSet,
}

#[derive(Serialize, Debug)]
pub struct ErrorStatus {
    pub message: String,
    pub status: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LogoutInfo {
    pub refresh_token: String,
    pub username: String,
}

#[derive(Serialize, FromRow)]
pub struct LogoutInfoId {
    pub id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct QueryReturnMessage {
    pub message: String,
}

impl FromRow<'_, MySqlRow> for RoleUser {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            role_user: row.try_get(0)?,
            // Map other fields...
        })
    }
}

impl FromRow<'_, MySqlRow> for AuthAdmin {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            username: row.try_get(1)?,
            password: row.try_get(2)?,
            token_salt: row.try_get(3)?,
            // Map other fields...
        })
    }
}
