use actix_web::{get, web::Data, HttpResponse, Responder};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use tracing::debug;

use crate::{
    admin::{CurrentAdminInfo, CurrentAdminInfoWithToken, CurrentAdminInfoWrapper},
    server_config, AdminContext, AppState, TokenClaims,
};

#[get("/info")]
pub async fn get_admin_info(state: Data<AppState>, ctx: AdminContext) -> impl Responder {
    let id = ctx.id;
    let query = "select id, firstname, middlename, lastname, email_address, username, role_user, token_salt from user_info_details where id = ?";

    let jwt_secret: Hmac<Sha256> =
        Hmac::new_from_slice(&server_config().JWT_SECRET.as_bytes()).unwrap();

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
            debug!("{:<12} - query error: {error:?}", "ERROR");
            HttpResponse::InternalServerError().json(format!("{error:?}"))
        }
    }
}
