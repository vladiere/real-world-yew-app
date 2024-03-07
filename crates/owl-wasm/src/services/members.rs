use crate::{error::Error, types::*};

use super::{request_delete, request_get};

pub async fn user_members(id: i64) -> Result<AllMembersInfoWrapper, Error> {
    request_get::<AllMembersInfoWrapper>(format!("/admin/user/member/all/{}", id)).await
}

pub async fn remove_member(id: i64) -> Result<QueryReturnMessage, Error> {
    request_delete::<QueryReturnMessage>(format!("/admin/user/member/{}", id)).await
}
