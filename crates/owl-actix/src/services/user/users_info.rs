use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use tracing::debug;

use crate::{
    user::{
        AccountsInfo, AccountsInfoWrapper, AllMembersInfo, AllMembersInfoWrapper, OneAccountInfo,
        OneAccountInfoWrapper,
    },
    AppState, ErrorStatus,
};

#[get("/{id}")]
pub async fn get_one_user(state: Data<AppState>, id: Path<i64>) -> impl Responder {
    let query = "select firstname, middlename, lastname, email_address, gender, recent_address, civil_status, occupation, tower, room, package, date_enrolled, user_id from user_info_details where role_user = 'User' and id = ?";

    debug!("{:<12} - get_one_user", "HANDLER");

    match sqlx::query_as::<_, OneAccountInfo>(query)
        .bind(*id)
        .fetch_one(&state.db)
        .await
    {
        Ok(account) => {
            let one_admin = OneAccountInfoWrapper { account };
            HttpResponse::Ok().json(one_admin)
        }
        Err(error) => {
            debug!("{:<12} - query error on get_one_user: {error:?}", "ERROR");
            let error = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}

#[get("/all/{id}")]
pub async fn user_members(state: Data<AppState>, id: Path<i64>) -> impl Responder {
    debug!("{:<12} - user_members", "HANDLER");

    let query = "select id, firstname, middlename, lastname, age, gender, date_format(ctime, '%M %e, %Y') as date_enrolled, member_id from members_table where user_id = ? and status = 'Active' order by ctime desc";

    match sqlx::query_as::<_, AllMembersInfo>(query)
        .bind(*id)
        .fetch_all(&state.db)
        .await
    {
        Ok(members) => {
            let all_members = AllMembersInfoWrapper { members };
            HttpResponse::Ok().json(all_members)
        }
        Err(error) => {
            debug!("{:<12} - query error on user_members: {error:?}", "ERROR");
            let err = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(err)
        }
    }
}

#[get("/all")]
pub async fn get_all_user(state: Data<AppState>) -> impl Responder {
    debug!("{:<12} - get_all_user", "HANDLER");

    let query = "select id, firstname, middlename, lastname, email_address, gender, tower, room, package, date_enrolled, status, user_id from user_info_details where role_user = 'User' and status != 'Removed' order by date_enrolled desc";

    match sqlx::query_as::<_, AccountsInfo>(query)
        .fetch_all(&state.db)
        .await
    {
        Ok(clients) => {
            let accounts = AccountsInfoWrapper { accounts: clients };
            HttpResponse::Ok().json(accounts)
        }
        Err(error) => {
            debug!("{:<12} - query error on get_all_user: {error:?}", "ERROR");
            let err = ErrorStatus {
                message: format!("Error at get_all_user query error: {error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(err)
        }
    }
}
