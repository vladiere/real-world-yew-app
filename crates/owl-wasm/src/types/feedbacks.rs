use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeedBacksInfo {
    pub name: String,
    pub message: String,
    pub date: DateTime<Utc>,
    pub time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeedBacksInfoWrapper {
    pub reports: FeedBacksInfo,
}
