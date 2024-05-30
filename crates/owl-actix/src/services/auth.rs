use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use tracing::debug;
use uuid::Uuid;

use crate::{
    admin::{CurrentAdminInfo, CurrentAdminInfoWithToken, CurrentAdminInfoWrapper},
    appstate::AppState,
    b32_hex, server_config, AuthAdmin, LogoutInfoId, LogoutInfoWrapper, QueryReturnMessage,
    RegisterAdminBodyWrapper, RoleUser,
};
use crate::{middleware::TokenClaims, ErrorStatus, LoginAdmin};

#[post("/register")]
pub async fn register_admin(
    state: Data<AppState>,
    body: Json<RegisterAdminBodyWrapper>,
) -> impl Responder {
    let user: RegisterAdminBodyWrapper = body.into_inner();
    let query = "CALL register_admin(?,?,?,?,?,?,?,?,?,?,?,?,?);";

    debug!(
        "{:<12} - register_admin: {}",
        user.admin.username, "ATTEMPTING"
    );

    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.admin.password)
        .with_secret_key(&server_config().HASH_SECRET)
        .hash()
        .unwrap();

    match sqlx::query(query)
        .bind(user.admin.firstname)
        .bind(user.admin.middlename)
        .bind(user.admin.lastname)
        .bind(user.admin.email_address)
        .bind(user.admin.gender)
        .bind(user.admin.recent_address)
        .bind(user.admin.civil_status)
        .bind(user.admin.occupation)
        .bind(user.admin.username)
        .bind(hash)
        .bind(Uuid::new_v4().to_string())
        .bind(Uuid::new_v4().to_string())
        .bind(match b32_hex() {
            Ok(val) => val,
            Err(_) => String::from("Null"),
        })
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Created successfully".to_string(),
            };
            HttpResponse::Ok().json(res)
        }
        Err(error) => {
            debug!("{:<12} - query error: {error:?}", "ERROR");
            let error = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}

#[post("/login")]
pub async fn login_admin(state: Data<AppState>, credentials: Json<LoginAdmin>) -> impl Responder {
    let query = "select id, username, password, token_salt from user_login where username = ?";
    let query2 = "select role_user from user_info where id = ?";
    let query3 = "insert into refresh_token (username,refresh_token) VALUES (?,?)";
    let query4 = "select id, firstname, middlename, lastname, email_address, username, role_user, token_salt from admin_info_details where id = ?";

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

                                    match sqlx::query_as::<_, CurrentAdminInfo>(query4)
                                        .bind(user.id)
                                        .fetch_one(&state.db)
                                        .await
                                    {
                                        Ok(admin) => {
                                            let current_admin_with_token =
                                                CurrentAdminInfoWithToken {
                                                    id: admin.id,
                                                    firstname: admin.firstname,
                                                    lastname: admin.lastname,
                                                    middlename: admin.middlename,
                                                    email_address: admin.email_address,
                                                    username: admin.username,
                                                    role_user: admin.role_user,
                                                    access_token: claims
                                                        .clone()
                                                        .sign_with_key(&jwt_secret)
                                                        .unwrap(),
                                                    refresh_token: claims
                                                        .sign_with_key(&jwt_secret)
                                                        .unwrap(),
                                                };

                                            let current_admin = CurrentAdminInfoWrapper {
                                                admin: current_admin_with_token,
                                            };
                                            match sqlx::query(query3)
                                                .bind(username)
                                                .bind(current_admin.admin.refresh_token.clone())
                                                .execute(&state.db)
                                                .await
                                            {
                                                Ok(_) => HttpResponse::Ok().json(current_admin),
                                                Err(error) => {
                                                    debug!( "{:<12} - query3 inserting token: {error:?}", "ERROR");
                                                    HttpResponse::InternalServerError()
                                                        .json(format!("{error:?}"))
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            debug!("{:<12} - query3 error {error:?}", "ERROR");
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
                message: String::from("Username is required"),
                status: 401,
            };
            HttpResponse::Unauthorized().json(error)
        }
    } else {
        let error = ErrorStatus {
            message: String::from("Username and Password are required"),
            status: 401,
        };
        HttpResponse::Unauthorized().json(error)
    }
}

#[post("/logout")]
pub async fn logout_admin(state: Data<AppState>, cred: Json<LogoutInfoWrapper>) -> impl Responder {
    let creds: LogoutInfoWrapper = cred.into_inner();
    let query1 = "select id from refresh_token where refresh_token = ? order by id desc limit 1";
    let query2 = "delete from refresh_token where username = ?";

    debug!("{:<12} - logging out {}", creds.admin.username, "LOGOUT");

    if creds.admin.username.is_empty() && creds.admin.refresh_token.is_empty() {
        let error = ErrorStatus {
            message: "Username and refresh_token is not set".to_string(),
            status: 401,
        };
        HttpResponse::Forbidden().json(error)
    } else {
        match sqlx::query_as::<_, LogoutInfoId>(query1)
            .bind(creds.admin.refresh_token)
            .fetch_one(&state.db)
            .await
        {
            Ok(_) => {
                match sqlx::query(query2)
                    .bind(creds.admin.username)
                    .execute(&state.db)
                    .await
                {
                    Ok(_) => {
                        let res = QueryReturnMessage {
                            message: "Logout successfully".to_string(),
                        };
                        HttpResponse::Ok().json(res)
                    }
                    Err(error) => {
                        debug!("{:<12} - logout_admin query2 {error:?}", "ERROR");
                        HttpResponse::InternalServerError().json(format!("{error:?}"))
                    }
                }
            }
            Err(error) => {
                debug!("{:<12} - logout_admin query1 {error:?}", "ERROR");
                HttpResponse::InternalServerError().json(format!("{error:?}"))
            }
        }
    }
}
