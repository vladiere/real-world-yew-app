use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use tracing::debug;

use crate::{
    admin::{
        AdminsInfo, AdminsInfoWrapper, CurrentAdminInfo, CurrentAdminInfoWithToken,
        CurrentAdminInfoWrapper, DashboardCount, DashboardCountWrapper, OneAdminInfo,
        OneAdminInfoWrapper,
    },
    server_config, AdminContext, AppState, ErrorStatus, TokenClaims,
};

#[get("/info")]
pub async fn get_admin_info(state: Data<AppState>, ctx: AdminContext) -> impl Responder {
    let id = ctx.id;
    let query = "select id, firstname, middlename, lastname, email_address, username, role_user, token_salt, user_id from admin_info_details where id = ?";

    let jwt_secret: Hmac<Sha256> =
        Hmac::new_from_slice(server_config().JWT_SECRET.as_bytes()).unwrap();

    debug!("{:<12} - get_admin_info", "HANDLER");
    match sqlx::query_as::<_, CurrentAdminInfo>(query)
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => {
            let claims = TokenClaims {
                id: user.id,
                username: (*user.username).to_string(),
                token_salt: user.token_salt,
                role_user: (*user.role_user).to_string(),
            };

            let current_admin_with_token = CurrentAdminInfoWithToken {
                id: user.id,
                firstname: user.firstname,
                lastname: user.lastname,
                middlename: user.middlename,
                email_address: user.email_address,
                username: user.username,
                role_user: user.role_user,
                access_token: claims.clone().sign_with_key(&jwt_secret).unwrap(),
                refresh_token: claims.sign_with_key(&jwt_secret).unwrap(),
            };

            let current_admin = CurrentAdminInfoWrapper {
                admin: current_admin_with_token,
            };

            HttpResponse::Ok().json(current_admin)
        }
        Err(error) => {
            debug!("{:<12} - query error on get_admin_info: {error:?}", "ERROR");
            HttpResponse::InternalServerError().json(format!("{error:?}"))
        }
    }
}

#[get("/{id}")]
pub async fn get_one_admin(state: Data<AppState>, id: Path<i64>) -> impl Responder {
    let query = "select firstname, middlename, lastname, email_address, gender, recent_address, civil_status, occupation, username, date_enrolled, status, user_id from admin_info_details where id = ? and role_user = 'Admin'";

    debug!("{:<12} - get_one_admin", "HANDLER");

    match sqlx::query_as::<_, OneAdminInfo>(query)
        .bind(*id)
        .fetch_one(&state.db)
        .await
    {
        Ok(admin) => {
            let one_admin = OneAdminInfoWrapper { admin };
            HttpResponse::Ok().json(one_admin)
        }
        Err(error) => {
            debug!("{:<12} - query error on get_one_admin: {error:?}", "ERROR");
            let error = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}

#[get("/all")]
pub async fn get_all_admin(state: Data<AppState>, ctx: AdminContext) -> impl Responder {
    let query = "select id, firstname, middlename, lastname, email_address, username, date_enrolled, status, user_id from admin_info_details where id != ? and role_user = 'Admin' and status != 'Removed'";

    debug!("{:<12} - get_all_admin", "HANDLER");

    match sqlx::query_as::<_, AdminsInfo>(query)
        .bind(ctx.id)
        .fetch_all(&state.db)
        .await
    {
        Ok(admins) => {
            let all_admin = AdminsInfoWrapper { admins };
            HttpResponse::Ok().json(all_admin)
        }
        Err(error) => {
            debug!("{:<12} - query error on get_all_admin: {error:?}", "ERROR");
            let error = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}

#[get("/dashboard")]
pub async fn get_dashboard_data(state: Data<AppState>) -> impl Responder {
    let query = r#"
        SELECT
            user_count.active_users,
            admin_count.admins,
            device_count.total_devices,
            monitor_count.monitors
        FROM
            (SELECT COUNT(*) AS active_users FROM user_info WHERE role_user = 'User' AND status = 'Active') AS user_count,
            (SELECT COUNT(*) AS admins FROM user_info WHERE role_user = 'Admin' AND status = 'Active') AS admin_count,
            (SELECT COUNT(*) AS total_devices FROM device_info) AS device_count,
            (SELECT COUNT(*) AS monitors FROM monitoring_table WHERE monitor_state = 'Opened') AS monitor_count;
        "#;

    debug!("{:<12} - get_dashboard_data", "HANDLER");

    match sqlx::query_as::<_, DashboardCount>(query)
        .fetch_one(&state.db)
        .await
    {
        Ok(data) => {
            let dashboard_counts = DashboardCountWrapper { data };
            HttpResponse::Ok().json(dashboard_counts)
        }
        Err(error) => {
            debug!(
                "{:<12} - query error on get_dashboard_data: {error:?}",
                "ERROR"
            );
            let error = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}
