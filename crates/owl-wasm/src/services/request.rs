use dotenv_codegen::dotenv;
use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;
use crate::types::{AuthorizeErrors, ErrorInfo};

const API_ROOT: &str = dotenv!("API_ROOT");
const TOKEN_ACCESS: &str = "access_token";
const TOKEN_REFRESH: &str = "refresh_token";

lazy_static! {
    ///JWT token read from local storage.
    pub static ref ACCESS_TOKEN: RwLock<Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_ACCESS) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
    pub static ref REFRESH_TOKEN: RwLock<Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_REFRESH) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

/// Set JWT token to local storage.
pub fn set_token(key: &str, token: Option<String>) {
    if !key.is_empty() {
        if let Some(t) = token.clone() {
            LocalStorage::set(key, t).expect("Failed to set token");
        } else {
            LocalStorage::delete(key);
        }
    }
    if key == "access_token" {
        let mut access_token_lock = ACCESS_TOKEN.write();
        *access_token_lock = token;
    } else if key == "refresh_token" {
        let mut refresh_token_lock = REFRESH_TOKEN.write();
        *refresh_token_lock = token;
    }
}

/// Get JWT token from lazy_static.
pub fn get_access() -> Option<String> {
    let token_lock = ACCESS_TOKEN.read();
    token_lock.clone()
}

pub fn get_refresh() -> Option<String> {
    let token_lock = REFRESH_TOKEN.read();
    token_lock.clone()
}

/// Build all kinds of http request: post/get/delet etc.
pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", API_ROOT, url);
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if let Some(token) = get_access() {
        builder = builder.bearer_auth(token);
    }

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                Ok(data)
            } else {
                log::debug!("Response: {data:?}");
                Err(Error::DeserializeError)
            }
        } else {
            match data.status().as_u16() {
                401 => {
                    let data: Result<AuthorizeErrors, _> = data.json::<AuthorizeErrors>().await;
                    if let Ok(data) = data {
                        Err(Error::Unauthorized(data.message))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                403 => Err(Error::Forbidden),
                404 => {
                    let data: Result<AuthorizeErrors, _> = data.json::<AuthorizeErrors>().await;
                    if let Ok(data) = data {
                        Err(Error::NotFound(data.message))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                500 => {
                    let data: Result<AuthorizeErrors, _> = data.json::<AuthorizeErrors>().await;
                    if let Ok(data) = data {
                        Err(Error::InternalServerError(data.message))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                422 => {
                    let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}

/// DELETE request
pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::DELETE, url, ()).await
}

/// GET request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, url, ()).await
}

/// POST request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::POST, url, body).await
}

/// PUT request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::PUT, url, body).await
}

/// Set limit for pagination
pub async fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
