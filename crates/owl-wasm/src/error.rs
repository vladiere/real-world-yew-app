//! Error type for error handling

use crate::types::ErrorInfo;
use thiserror::Error as ThisError;

///Define all possible errors
#[derive(ThisError, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    ///401
    #[error("{0}")]
    Unauthorized(String),

    ///403
    #[error("Forbidden")]
    Forbidden,

    ///404
    #[error("{0}")]
    NotFound(String),

    ///422
    #[error("Unprocessable Entity {0:?}")]
    UnprocessableEntity(ErrorInfo),

    ///500
    #[error("{0}")]
    InternalServerError(String),

    ///serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    ///request error
    #[error("HTTP Request Error")]
    RequestError,
}
