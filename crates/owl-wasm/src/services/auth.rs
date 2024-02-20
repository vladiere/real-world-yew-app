use super::{request_get, request_post, request_put};
use crate::{error::Error, types::*};

/// GET current admin info.
pub async fn current() -> Result<CurrentAdminInfoWrapper, Error> {
    request_get::<CurrentAdminInfoWrapper>("/admin/info".to_string()).await
}

/// Login a admin
pub async fn login(login_info: LoginInfoWrapper) -> Result<CurrentAdminInfoWrapper, Error> {
    request_post::<LoginInfoWrapper, CurrentAdminInfoWrapper>(
        "/admin/login".to_string(),
        login_info,
    )
    .await
}

/// Register a new admin
pub async fn register_admin(
    register_info: AdminRegisterInfoWrapper,
) -> Result<AdminInfoWrapper, Error> {
    request_post::<AdminRegisterInfoWrapper, AdminInfoWrapper>(
        "/admin/register".to_string(),
        register_info,
    )
    .await
}

/// Logout current admin
pub async fn logout_admin(admin_info: LogoutInfoWrapper) -> Result<LogoutInfoWrapper, Error> {
    request_post::<LogoutInfoWrapper, LogoutInfoWrapper>("/admin/logout".to_string(), admin_info)
        .await
}

/// Get the admin info
pub async fn get_info_admin() -> Result<AdminRegisterInfoWrapper, Error> {
    request_get::<AdminRegisterInfoWrapper>("/admin/info".to_string()).await
}

/// Save info of current admin
pub async fn save(admin_update_info: AdminUpdateInfoWrapper) -> Result<AdminInfoWrapper, Error> {
    request_put::<AdminUpdateInfoWrapper, AdminInfoWrapper>(
        "/admin/update".to_string(),
        admin_update_info,
    )
    .await
}
