//! Common types

mod accounts;
mod auth;
mod devices;
mod errors;
mod feedbacks;
mod monitoring;
mod profile;
mod reports;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use accounts::{
    AccountRegisterInfo, AccountRegisterInfoWrapper, AccountsInfo, AccountsInfoWrapper,
    AllAdminInfo, AllAdminInfoWrapper, AllMembersInfo, AllMembersInfoWrapper, MemberInfo,
    MemberInfoWrapper, OneAccountInfo, OneAccountInfoWrapper, OneAdminInfo, OneAdminInfoWrapper,
    UpdateOneAdminInfo, UpdateOneAdminInfoWrapper,
};
pub use auth::{
    AdminInfo, AdminInfoWrapper, AdminRegisterInfo, AdminRegisterInfoWrapper, AdminUpdateInfo,
    AdminUpdateInfoWrapper, CurrentAdminInfo, CurrentAdminInfoWrapper, LoginInfo, LoginInfoWrapper,
    LogoutInfo, LogoutInfoWrapper, QueryReturnMessage, UserRegisterInfo, UserRegisterInfoWrapper,
};
pub use devices::{
    DeviceForCreate, DeviceForCreateWrapper, DeviceForSelect, DeviceForSelectOne,
    DeviceForSelectOneWrapper, DeviceForSelectWrapper, DeviceForUpdate, DeviceForUpdateWrapper,
};
pub use errors::AuthorizeErrors;
pub use monitoring::{
    MonitorForSelect, MonitorForSelectOne, MonitorForSelectOneWrapper, MonitorForSelectWrapper,
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
