use actix_web::{
    delete, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use argonautica::Hasher;
use tracing::debug;

use crate::{
    admin::{AdminUpdate, UpdateOneAdminWrapper},
    server_config, AdminContext, AppState, ErrorStatus, QueryReturnMessage,
};

#[post("/one/update")]
pub async fn update_one_admin(
    state: Data<AppState>,
    body: Json<UpdateOneAdminWrapper>,
) -> impl Responder {
    let query1 = "update user_login set username = ? where user_id = ?";
    let query2 = "update user_info set email_address = ? where id = ?";

    debug!("{:<12} - one update_admin", "HANDLER");

    let user: UpdateOneAdminWrapper = body.into_inner();

    match sqlx::query(query1)
        .bind(user.admin.username)
        .bind(&user.admin.admin_id)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            match sqlx::query(query2)
                .bind(user.admin.email_address)
                .bind(user.admin.admin_id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    let res = QueryReturnMessage {
                        message: "Updated successfully".to_string(),
                    };
                    HttpResponse::Ok().json(res)
                }
                Err(error) => {
                    debug!(
                        "{:<12} - query2 error on update_one_admin: {error:?}",
                        "ERROR"
                    );
                    let err = ErrorStatus {
                        message: format!("{error:?}"),
                        status: 500,
                    };
                    HttpResponse::InternalServerError().json(err)
                }
            }
        }
        Err(error) => {
            debug!(
                "{:<12} - query1 error on update_one_admin: {error:?}",
                "ERROR"
            );
            let err = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(err)
        }
    }
}

#[post("/update")]
pub async fn admin_update(
    state: Data<AppState>,
    ctx: AdminContext,
    body: Json<AdminUpdate>,
) -> impl Responder {
    let query = "update user_login set password = ? where user_id = ?";

    debug!("{:<12} - admin_update", "HANDLER");

    let admin: AdminUpdate = body.into_inner();

    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(admin.password)
        .with_secret_key(&server_config().HASH_SECRET)
        .hash()
        .unwrap();

    match sqlx::query(query)
        .bind(hash)
        .bind(ctx.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Password updated successfully!".to_string(),
            };
            HttpResponse::Ok().json(res)
        }
        Err(error) => {
            debug!("{:<12} - query error on admin_update: {error:?}", "ERROR");
            let error = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}

#[delete("/remove/{id}")]
pub async fn remove_one_admin(state: Data<AppState>, id: Path<i64>) -> impl Responder {
    let query = "update user_info set status = 'Removed' where id = ?";

    debug!("{:<12} - remove_one_admin", "HANDLER");

    match sqlx::query(query).bind(*id).execute(&state.db).await {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Admin removed successfully".to_string(),
            };
            HttpResponse::Ok().json(res)
        }
        Err(error) => {
            debug!(
                "{:<12} - query error on remove_one_admin: {error:?}",
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
