use actix_web::{
    delete, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use tracing::debug;

use crate::{
    user::{AccountRegisterInfoWrapper, MemberInfoWrapper},
    AppState, ErrorStatus, QueryReturnMessage,
};

#[post("/register")]
pub async fn register_user(
    state: Data<AppState>,
    body: Json<AccountRegisterInfoWrapper>,
) -> impl Responder {
    debug!("{:<12} - register_user", "HANDLER");

    let query = "call register_user(?,?,?,?,?,?,?,?,?,?,?)";
    let acc: AccountRegisterInfoWrapper = body.into_inner();

    match sqlx::query(query)
        .bind(acc.account.firstname)
        .bind(acc.account.middlename)
        .bind(acc.account.lastname)
        .bind(acc.account.email_address)
        .bind(acc.account.gender)
        .bind(acc.account.recent_address)
        .bind(acc.account.civil_status)
        .bind(acc.account.occupation)
        .bind(acc.account.tower)
        .bind(acc.account.room)
        .bind(acc.account.package)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Successfully register client".to_string(),
            };
            HttpResponse::Ok().json(res)
        }
        Err(error) => {
            debug!(
                "{:<12} - Error at register_user query error: {error:?}",
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

#[delete("/remove/{id}")]
pub async fn remove_one_user(state: Data<AppState>, id: Path<i64>) -> impl Responder {
    debug!("{:<12} - remove_user", "HANDLER");

    let query = "update user_info set status = 'Removed' where id = ?";
    match sqlx::query(query).bind(*id).execute(&state.db).await {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Client removed successfully".to_string(),
            };
            HttpResponse::Ok().json(res)
        }
        Err(error) => {
            let err = ErrorStatus {
                message: format!("Removed user error: {error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(err)
        }
    }
}

#[post("/add")]
pub async fn add_member(state: Data<AppState>, body: Json<MemberInfoWrapper>) -> impl Responder {
    debug!("{:<12} - add_member", "HANDLER");

    let query = "insert into members_table (firstname,middlename,lastname,age,gender,user_id) values (?,?,?,?,?,?)";
    let member: MemberInfoWrapper = body.into_inner();

    match sqlx::query(query)
        .bind(member.member.firstname)
        .bind(member.member.middlename)
        .bind(member.member.lastname)
        .bind(member.member.age)
        .bind(member.member.gender)
        .bind(member.member.user_id)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Member added successfully".to_string(),
            };
            HttpResponse::Ok().json(res)
        }
        Err(error) => {
            debug!("{:<12} - query error on add_member: {error:?}", "ERROR");
            let err = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(err)
        }
    }
}

#[delete("/{id}")]
pub async fn remove_member(state: Data<AppState>, id: Path<i64>) -> impl Responder {
    debug!("{:<12} - remove_member", "HANDLER");

    let query = "update members_table set status = 'Removed' where id = ?";

    match sqlx::query(query).bind(*id).execute(&state.db).await {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Member removed successfully".to_string(),
            };
            HttpResponse::Ok().json(res)
        }
        Err(error) => {
            debug!("{:<12} - query error on remove_member: {error:?}", "ERROR");
            let err = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(err)
        }
    }
}
