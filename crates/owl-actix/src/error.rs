use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    CannotCompileB32,
    CannotConvert { kind: String, message: String },
    MissingEnv(&'static str),
    WrongFormat(&'static str),
    CannotFrom,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
