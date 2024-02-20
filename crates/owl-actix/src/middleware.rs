use std::ops::Deref;

use actix_web::{dev::ServiceRequest, error::Error, FromRequest, HttpMessage, HttpRequest};
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::server_config;

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: i64,
    pub role_user: String,
    pub username: String,
    pub token_salt: String,
}

// Custom context struct containing TokenClaims
pub struct AdminContext {
    pub claims: TokenClaims,
}

// Implement Deref for AdminContext to easily access TokenClaims
impl Deref for AdminContext {
    type Target = TokenClaims;

    fn deref(&self) -> &Self::Target {
        &self.claims
    }
}

// Custom admin extractor for AdminContext
pub struct AdminContextExtractor;

// Implement FromRequest for AdminContextExtractor
impl FromRequest for AdminContext {
    type Error = Error;
    type Future = futures_util::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // Extract TokenClaims from request extensions
        if let Some(claims) = req.extensions().get::<TokenClaims>() {
            // Create and return AdminContext
            futures_util::future::ok(AdminContext {
                claims: claims.clone(),
            })
        } else {
            // If no claims are found, return an error or a default context
            futures_util::future::err(actix_web::error::ErrorUnauthorized("Unauthorized"))
        }
    }
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(&server_config().JWT_SECRET.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
