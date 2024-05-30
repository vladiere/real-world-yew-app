use crate::{envs::get_env, get_env_parse};
use std::sync::OnceLock;

pub fn server_config() -> &'static ServerConfig {
    static INSTANCE: OnceLock<ServerConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        ServerConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct ServerConfig {
    pub DB_URL: String,
    pub JWT_SECRET: String,
    pub HASH_SECRET: String,
    pub DB_NAME: String,
    pub DB_USER: String,
    pub DB_PASS: String,
    pub DB_HOST: String,
    pub DB_PORT: i64,
}

impl ServerConfig {
    fn load_from_env() -> crate::error::Result<ServerConfig> {
        Ok(ServerConfig {
            JWT_SECRET: get_env("JWT_SECRET")?,
            DB_URL: get_env("SERVICE_DB_URL")?,
            HASH_SECRET: get_env("HASH_SECRET")?,
            DB_NAME: get_env("DB_NAME")?,
            DB_USER: get_env("DB_USER")?,
            DB_PASS: get_env("DB_PASS")?,
            DB_HOST: get_env("DB_HOST")?,
            DB_PORT: get_env_parse("DB_PORT")?,
        })
    }
}
