//! Common types

mod accounts;
mod auth;
mod errors;
mod feedbacks;
mod monitoring;
mod profile;
mod reports;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use accounts::{
    AccountRegisterInfo, AccountRegisterInfoWrapper, AccountsInfo, AccountsInfoWrapper,
};
pub use auth::{
    AdminInfo, AdminInfoWrapper, AdminRegisterInfo, AdminRegisterInfoWrapper, AdminUpdateInfo,
    AdminUpdateInfoWrapper, CurrentAdminInfo, CurrentAdminInfoWrapper, LoginInfo, LoginInfoWrapper,
    LogoutInfo, LogoutInfoWrapper, QueryReturnMessage, UserRegisterInfo, UserRegisterInfoWrapper,
};
pub use errors::AuthorizeErrors;
pub use monitoring::{
    MonitoringInfo, MonitoringInfoWrapper, ViewMonitoringInfo, ViewMonitoringInfoWrapper,
};
pub use profile::{ProfileInfo, ProfileInfoWrapper};
pub use reports::{ReportsInfo, ReportsInfoWrapper};

/// OWL api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type DeleteWrapper = HashMap<(), ()>;
