use crate::envs::get_env;
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
}

impl ServerConfig {
    fn load_from_env() -> crate::envs::Result<ServerConfig> {
        Ok(ServerConfig {
            JWT_SECRET: get_env("JWT_SECRET")?,
            DB_URL: get_env("SERVICE_DB_URL")?,
            HASH_SECRET: get_env("HASH_SECRET")?,
        })
    }
}
