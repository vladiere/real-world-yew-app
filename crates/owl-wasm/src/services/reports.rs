use super::{request_get, request_post};
use crate::error::Error;
use crate::types::{DashboardCountWrapper, ReportsInfoWrapper};

/// POST a reports.
pub async fn send_report(reports_info: ReportsInfoWrapper) -> Result<ReportsInfoWrapper, Error> {
    request_post::<ReportsInfoWrapper, ReportsInfoWrapper>(
        "/admin/report".to_string(),
        reports_info,
    )
    .await
}

/// GET dashboard counts.
pub async fn get_dashboard_data() -> Result<DashboardCountWrapper, Error> {
    request_get::<DashboardCountWrapper>("/admin/dashboard".to_string()).await
}
