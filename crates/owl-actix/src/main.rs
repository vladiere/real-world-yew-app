use actix_cors::Cors;
use actix_web::{
    get, http,
    web::{self, Data},
    App, HttpServer, Responder,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use owl_actix::{login_admin, logout_admin, register_admin, server_config, validator, AppState};
use sqlx::mysql::MySqlPoolOptions;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db_url = &server_config().DB_URL;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Error builing a connection pool");

    info!("{:<12} - http://127.0.0.1:8080", "LISTENING");
    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(validator);

        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(
                web::scope("/api").service(greet).service(
                    web::scope("/admin")
                        .service(register_admin)
                        .service(login_admin)
                        .service(
                            web::scope("")
                                .wrap(bearer_middleware)
                                .service(auth_get)
                                .service(logout_admin),
                        ),
                ),
            )
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello! {name}")
}

#[get("/api/test")]
async fn auth_get() -> impl Responder {
    format!("Authenticated")
}
