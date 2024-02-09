use web_sys::{console, js_sys::JsString};

use super::{get_token, request_get, request_post, request_put};
use crate::{error::Error, types::*};

/// GET current admin info.
pub async fn current() -> Result<AdminInfoWrapper, Error> {
    if let Some(token) = get_token() {
        console::log_1(&JsString::from(format!("{}", token)));
    }
    request_get::<AdminInfoWrapper>("/admin/info".to_string()).await
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

/// Save info of current admin
pub async fn save(admin_update_info: AdminUpdateInfoWrapper) -> Result<AdminInfoWrapper, Error> {
    request_put::<AdminUpdateInfoWrapper, AdminInfoWrapper>(
        "/admin/update".to_string(),
        admin_update_info,
    )
    .await
}
