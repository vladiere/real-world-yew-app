use super::request_get;
use crate::{
    error::Error,
    types::{MonitorForSelectOneWrapper, MonitorForSelectWrapper},
};

/// GET all monitorings.
pub async fn monitoring_select() -> Result<MonitorForSelectWrapper, Error> {
    request_get("/monitoring/all".to_string()).await
}

/// GET single monitoring.
pub async fn monitoring_select_one(monitor_id: u32) -> Result<MonitorForSelectOneWrapper, Error> {
    request_get(format!("/monitoring/one/{}", monitor_id)).await
}
