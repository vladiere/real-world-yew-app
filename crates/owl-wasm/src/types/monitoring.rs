use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringInfo {
    pub client_number: i32,
    pub name: String,
    pub date: DateTime<Utc>,
    pub door_opened: DateTime<Utc>,
    pub door_closed: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringInfoWrapper {
    pub monitor: MonitoringInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewMonitoringInfo {
    pub date: String,
    pub time_in: DateTime<Utc>,
    pub time_out: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewMonitoringInfoWrapper {
    pub views: ViewMonitoringInfo,
}
