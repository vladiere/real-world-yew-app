use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

pub mod admin;
pub mod admin_info;
pub mod device;
pub mod monitoring;

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
pub struct DashboardCount {
    pub active_users: i64,
    pub admins: i64,
    pub total_devices: i64, 
    pub monitors: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DashboardCountWrapper {
    pub data: DashboardCount,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForCreate {
    pub device_name: String,
    pub device_tower: String,
    pub device_room: String,
    pub device_state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForCreateWrapper {
    pub device: DeviceForCreate,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForSelectWrapper {
    pub devices: Vec<DeviceForSelect>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForUpdate {
    pub id: i64,
    pub device_state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceForUpdateWrapper {
    pub device: DeviceForUpdate,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MonitorForSelect {
    pub id: i64,
    pub client_name: String,
    pub building_tower: String,
    pub building_room: String,
    pub device_state: String,
    pub date_begin: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MonitorForSelectWrapper {
    pub monitor: Vec<MonitorForSelect>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MonitorForSelectOne {
    pub id: i64,
    pub client_name: String,
    pub building_tower: String,
    pub building_room: String,
    pub device_state: String,
    pub date_begin: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MonitorForSelectOneWrapper {
    pub monitor: MonitorForSelectOne,
}

impl FromRow<'_, MySqlRow> for MonitorForSelectOne {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            client_name: row.try_get(1)?,
            building_tower: row.try_get(2)?,
            building_room: row.try_get(3)?,
            device_state: row.try_get(4)?,
            date_begin: row.try_get(5)?,
            date_modified: row.try_get(6)?,
        })
    }
}

impl FromRow<'_, MySqlRow> for MonitorForSelect {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            client_name: row.try_get(1)?,
            building_tower: row.try_get(2)?,
            building_room: row.try_get(3)?,
            device_state: row.try_get(4)?,
            date_begin: row.try_get(5)?,
            date_modified: row.try_get(6)?,
        })
    }
}

impl FromRow<'_, MySqlRow> for DeviceForSelectOne {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            device_name: row.try_get(1)?,
            device_tower: row.try_get(2)?,
            device_room: row.try_get(3)?,
            device_state: row.try_get(4)?,
            created_at: row.try_get(5)?,
            modified_at: row.try_get(6)?,
        })
    }
}

impl FromRow<'_, MySqlRow> for DeviceForSelect {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            device_name: row.try_get(1)?,
            device_tower: row.try_get(2)?,
            device_room: row.try_get(3)?,
            device_state: row.try_get(4)?,
            created_at: row.try_get(5)?,
            modified_at: row.try_get(6)?,
        })
    }
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

impl FromRow<'_, MySqlRow> for DashboardCount {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self { active_users: row.try_get(0)?, admins: row.try_get(1)?, total_devices: row.try_get(2)?, monitors: row.try_get(3)? })
    }
}
