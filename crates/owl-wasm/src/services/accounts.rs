use super::{request_get, request_post};
use crate::{error::Error, types::*};

/// GET all users.
pub async fn get_all_users() -> Result<AccountsInfoWrapper, Error> {
    request_get("/user/get".to_string()).await
}

/// GET single user.
pub async fn get_single_user(user_id: u32) -> Result<AccountsInfoWrapper, Error> {
    request_get(format!("/user/get?id={}", user_id)).await
}

/// POST new account/user/client.
pub async fn add_user(user_info: AccountRegisterInfoWrapper) -> Result<AccountsInfoWrapper, Error> {
    request_post::<AccountRegisterInfoWrapper, AccountsInfoWrapper>(
        "/user/register".to_string(),
        user_info,
    )
    .await
}

/// DELETE a account/user/client.
pub async fn delete_user(acc_id: u32) -> Result<AccountsInfoWrapper, Error> {
    request_post("/user/delete".to_string(), acc_id).await
}

/// UPDATE a account/user/client.
pub async fn update_user(
    update_info: AccountRegisterInfoWrapper,
) -> Result<AccountsInfoWrapper, Error> {
    request_post("/user/update".to_string(), update_info).await
}
