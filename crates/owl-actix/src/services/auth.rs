use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde_json::json;
use sha2::Sha256;
use sqlx;
use tracing::debug;
use uuid::Uuid;

use crate::{
    appstate::AppState, server_config, AdminNoPassword, AuthAdmin, LogoutInfo, LogoutInfoId,
    QueryReturnMessage, RoleUser,
};
use crate::{
    middleware::TokenClaims, ErrorStatus, LoginAdmin, RegisterAdminBody, ResponseToken, TokenSet,
};

#[post("/register")]
pub async fn register_admin(
    state: Data<AppState>,
    body: Json<RegisterAdminBody>,
) -> impl Responder {
    let user: RegisterAdminBody = body.into_inner();
    let query = "CALL create_user_or_admin(?,?,?,?,?,?,?,?,?,?,?,?);";

    debug!("{:<12} - register_admin: {}", user.username, "ATTEMPTING");

    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(&server_config().HASH_SECRET)
        .hash()
        .unwrap();

    match sqlx::query(query)
        .bind(user.firstname)
        .bind(user.middlename)
        .bind(user.lastname)
        .bind(user.tower)
        .bind(user.occupation)
        .bind(user.position)
        .bind(user.email_address)
        .bind(user.contact_number)
        .bind(user.username)
        .bind(hash)
        .bind(Uuid::new_v4())
        .bind(Uuid::new_v4())
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Created successfully".to_string(),
            };
            HttpResponse::InternalServerError().json(res)
        }
        Err(error) => {
            debug!("{:<12} - query error: {error:?}", "ERROR");
            HttpResponse::InternalServerError().json(format!("{error:?}"))
        }
    }
}

#[post("/login")]
pub async fn login_admin(state: Data<AppState>, credentials: Json<LoginAdmin>) -> impl Responder {
    let query = "SELECT id, username, password, token_salt from user_login WHERE username = ?";
    let query2 = "SELECT role_user FROM user_info WHERE id = ?";
    let query3 = "INSERT INTO refresh_token (username,refresh_token) VALUES (?,?)";

    let jwt_secret: Hmac<Sha256> =
        Hmac::new_from_slice(&server_config().JWT_SECRET.as_bytes()).unwrap();

    let user: LoginAdmin = credentials.into_inner();
    let username = user.user.username;
    let pass = user.user.password;

    debug!("{:<12} - login_admin {username:?}", "ATTEMPTING");
    if !username.is_empty() && !pass.is_empty() {
        if !username.is_empty() {
            if !pass.is_empty() {
                match sqlx::query_as::<_, AuthAdmin>(query)
                    .bind(username.clone())
                    .fetch_one(&state.db)
                    .await
                {
                    Ok(user) => {
                        let mut verifier = Verifier::default();
                        let is_valid = verifier
                            .with_hash(user.password)
                            .with_password(pass)
                            .with_secret_key(&server_config().HASH_SECRET)
                            .verify()
                            .unwrap();

                        if is_valid {
                            match sqlx::query_as::<_, RoleUser>(query2)
                                .bind(user.id)
                                .fetch_one(&state.db)
                                .await
                            {
                                Ok(res) => {
                                    let claims = TokenClaims {
                                        id: user.id,
                                        username: user.username,
                                        token_salt: user.token_salt,
                                        role_user: res.role_user,
                                    };

                                    let token_set = TokenSet {
                                        access_token: claims
                                            .clone()
                                            .sign_with_key(&jwt_secret)
                                            .unwrap(),
                                        refresh_token: claims.sign_with_key(&jwt_secret).unwrap(),
                                    };
                                    let token_str = ResponseToken { admin: token_set };

                                    match sqlx::query(query3)
                                        .bind(username)
                                        .bind(token_str.admin.refresh_token.clone())
                                        .execute(&state.db)
                                        .await
                                    {
                                        Ok(_) => HttpResponse::Ok().json(token_str),
                                        Err(error) => {
                                            debug!(
                                                "{:<12} - query3 inserting token: {error:?}",
                                                "ERROR"
                                            );
                                            HttpResponse::InternalServerError()
                                                .json(format!("{error:?}"))
                                        }
                                    }
                                }
                                Err(error) => {
                                    debug!("{:<12} - query2 login {error:?}", "ERROR");
                                    HttpResponse::InternalServerError().json(format!("{error:?}"))
                                }
                            }
                        } else {
                            let error = ErrorStatus {
                                message: String::from("Incorrect username or password"),
                                status: 401,
                            };
                            HttpResponse::Unauthorized().json(error)
                        }
                    }
                    Err(error) => {
                        debug!("{:<12} - error login {error:?}", "ERROR");
                        let err_msg = ErrorStatus {
                            message: "Username does not exist".to_string(),
                            status: 404,
                        };
                        HttpResponse::NotFound().json(err_msg)
                    }
                }
            } else {
                let error = ErrorStatus {
                    message: String::from("Password is required"),
                    status: 401,
                };
                HttpResponse::Unauthorized().json(error)
            }
        } else {
            let error = ErrorStatus {
                message: String::from("Email address is required"),
                status: 401,
            };
            HttpResponse::Unauthorized().json(error)
        }
    } else {
        let error = ErrorStatus {
            message: String::from("Email address and password are required"),
            status: 401,
        };
        HttpResponse::Unauthorized().json(error)
    }
}

#[post("/logout")]
pub async fn logout_admin(state: Data<AppState>, cred: Json<LogoutInfo>) -> impl Responder {
    let creds: LogoutInfo = cred.into_inner();
    let query1 = "select id from refresh_token where refresh_token = $1 order by id desc limit 1";
    let query2 = "delete from refresh_token where username = $1";

    debug!("{:<12} - logging out {}", creds.username, "LOGOUT");

    if creds.username.is_empty() && creds.refresh_token.is_empty() {
        let error = ErrorStatus {
            message: "Username and refresh_token is not set".to_string(),
            status: 401,
        };
        HttpResponse::Forbidden().json(error)
    } else {
        match sqlx::query_as::<_, LogoutInfoId>(query1)
            .bind(creds.refresh_token)
            .fetch_one(&state.db)
            .await
        {
            Ok(_) => {
                match sqlx::query(query2)
                    .bind(creds.username)
                    .execute(&state.db)
                    .await
                {
                    Ok(_) => {
                        let result = json!({
                            "message": "logout successfully"
                        });
                        HttpResponse::Ok().json(result)
                    }
                    Err(error) => {
                        debug!("{:<12} - logout_admin query2 login {error:?}", "ERROR");
                        HttpResponse::InternalServerError().json(format!("{error:?}"))
                    }
                }
            }
            Err(error) => {
                debug!("{:<12} - logout_admin query login {error:?}", "ERROR");
                HttpResponse::InternalServerError().json(format!("{error:?}"))
            }
        }
    }
}
