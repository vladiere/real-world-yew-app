use web_sys::{console, js_sys::JsString};

use super::{get_access, get_refresh, request_get, request_post, request_put};
use crate::{error::Error, types::*};

/// GET current admin info.
pub async fn current() -> Result<AdminInfoWrapper, Error> {
    let mut admin_info = AdminInfo::default();
    if let Some(access) = get_access() {
        admin_info.access_token = access;
    }
    if let Some(refresh) = get_refresh() {
        admin_info.refresh_token = refresh;
    }

    Ok(AdminInfoWrapper { admin: admin_info })
}

/// Login a admin
pub async fn login(login_info: LoginInfoWrapper) -> Result<AdminInfoWrapper, Error> {
    request_post::<LoginInfoWrapper, AdminInfoWrapper>("/admin/login".to_string(), login_info).await
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
