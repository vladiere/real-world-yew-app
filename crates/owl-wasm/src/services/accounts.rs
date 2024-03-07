use super::{request_delete, request_get, request_post};
use crate::{error::Error, types::*};

/// GET all users.
pub async fn get_all_users() -> Result<AccountsInfoWrapper, Error> {
    request_get::<AccountsInfoWrapper>("/admin/user/all".to_string()).await
}

/// GET one user.
pub async fn get_one_user(id: i64) -> Result<OneAccountInfoWrapper, Error> {
    request_get::<OneAccountInfoWrapper>(format!("/admin/user/{}", id)).await
}

/// GET all admins.
pub async fn get_all_admins() -> Result<AllAdminInfoWrapper, Error> {
    request_get::<AllAdminInfoWrapper>("/admin/all".to_string()).await
}

/// GET one admin.
pub async fn get_one_admin(id: i64) -> Result<OneAdminInfoWrapper, Error> {
    request_get::<OneAdminInfoWrapper>(format!("/admin/{}", id)).await
}

/// POST new account/user/client.
pub async fn add_user(user_info: AccountRegisterInfoWrapper) -> Result<QueryReturnMessage, Error> {
    request_post::<AccountRegisterInfoWrapper, QueryReturnMessage>(
        "/admin/user/register".to_string(),
        user_info,
    )
    .await
}

/// DELETE a account/user/client.
pub async fn delete_user(acc_id: i64) -> Result<QueryReturnMessage, Error> {
    request_delete::<QueryReturnMessage>(format!("/admin/user/remove/{}", acc_id)).await
}

/// DELETE an admin.
pub async fn remove_one_admin(id: i64) -> Result<QueryReturnMessage, Error> {
    request_delete::<QueryReturnMessage>(format!("/admin/remove/{}", id)).await
}

/// UPDATE a account/user/client.
pub async fn update_user(
    update_info: AccountRegisterInfoWrapper,
) -> Result<AccountsInfoWrapper, Error> {
    request_post("/user/update".to_string(), update_info).await
}

/// UPDATE a one admin
pub async fn update_one_admin(
    update_info: UpdateOneAdminInfoWrapper,
) -> Result<QueryReturnMessage, Error> {
    request_post::<UpdateOneAdminInfoWrapper, QueryReturnMessage>(
        "/admin/one/update".to_string(),
        update_info,
    )
    .await
}

/// ADD new member for the specific client.
pub async fn add_new_member(member: MemberInfoWrapper) -> Result<QueryReturnMessage, Error> {
    request_post::<MemberInfoWrapper, QueryReturnMessage>(
        "/admin/user/member/add".to_string(),
        member,
    )
    .await
}
