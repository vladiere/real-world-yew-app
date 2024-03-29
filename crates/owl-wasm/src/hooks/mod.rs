use dotenv_codegen::dotenv;

mod decode_token;
mod match_email;
mod use_user_context;

// pub use decode_token::*;
pub use match_email::*;
pub use use_user_context::*;

pub const JWT_SECRET: &str = dotenv!("JWT_SECRET");
