use super::request_get;
use crate::{
    error::Error,
    types::{MonitoringInfoWrapper, ViewMonitoringInfoWrapper},
};

/// GET all monitorings.
pub async fn get_all_monitorings() -> Result<MonitoringInfoWrapper, Error> {
    request_get("/monitoring/get".to_string()).await
}

/// GET single monitoring.
pub async fn get_single_monitorings(monitor_id: u32) -> Result<ViewMonitoringInfoWrapper, Error> {
    request_get(format!("/monitoring/get?monitor_id={}", monitor_id)).await
}
