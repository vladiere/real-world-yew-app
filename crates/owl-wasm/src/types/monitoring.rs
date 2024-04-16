use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MonitorForSelectWrapper {
    pub monitors: Vec<MonitorForSelect>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MonitorForSelectOneWrapper {
    pub monitor: MonitorForSelectOne,
}
