use super::request_post;
use crate::error::Error;
use crate::types::ReportsInfoWrapper;

/// POST a reports.
pub async fn send_report(reports_info: ReportsInfoWrapper) -> Result<ReportsInfoWrapper, Error> {
    request_post::<ReportsInfoWrapper, ReportsInfoWrapper>(
        "/admin/report".to_string(),
        reports_info,
    )
    .await
}
