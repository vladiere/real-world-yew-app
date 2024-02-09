use hmac::{Hmac, Mac};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sha2::Sha256;
use tracing::debug;

use crate::{server_config, TokenClaims};

pub fn extractor_token(token: &str) -> Result<TokenClaims> {
    let _key: Hmac<Sha256> = Hmac::new_from_slice(&server_config().JWT_SECRET.as_bytes()).unwrap();
    let decoded_key = DecodingKey::from_secret(&server_config().JWT_SECRET.as_bytes());

    match decode::<TokenClaims>(token, &decoded_key, &Validation::default()) {
        Ok(c) => Ok(c.claims),
        Err(error) => {
            debug!("{:<12} - extractor_token {error:?}", "ERROR");
            Err(Error::CannotDecodeWrongFormat)
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CannotDecodeWrongFormat,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
