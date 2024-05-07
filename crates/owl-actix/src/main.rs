use actix_cors::Cors;
use actix_web::{
    get, http,
    web::{self, Data},
    App, HttpServer, Responder,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use owl_actix::{
    admin::{
        admin::{admin_update, remove_one_admin, update_one_admin},
        admin_info::{get_admin_info, get_all_admin, get_dashboard_data, get_one_admin},
        device::{create_device, delete_device, select_devices, select_one_device, update_device},
        monitoring::{monitor_select, monitor_select_one},
    },
    login_admin, logout_admin, register_admin, server_config,
    user::{
        users::{add_member, register_user, remove_member, remove_one_user},
        users_info::{get_all_user, get_one_user, user_members},
    },
    validator, AdminContextExtractor, AppState,
};
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

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&server_config().DB_URL)
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
            .app_data(Data::new(AdminContextExtractor))
            .service(
                web::scope("/api").service(greet).service(
                    web::scope("/admin").service(login_admin).service(
                        web::scope("")
                            .wrap(bearer_middleware)
                            .service(get_dashboard_data)
                            .service(get_admin_info)
                            .service(get_all_admin)
                            .service(get_one_admin)
                            .service(logout_admin)
                            .service(register_admin)
                            .service(admin_update)
                            .service(update_one_admin)
                            .service(remove_one_admin)
                            .service(
                                web::scope("/monitoring")
                                    .service(monitor_select)
                                    .service(monitor_select_one),
                            )
                            .service(
                                web::scope("/device")
                                    .service(select_devices)
                                    .service(select_one_device)
                                    .service(create_device)
                                    .service(update_device)
                                    .service(delete_device),
                            )
                            .service(
                                web::scope("/user")
                                    .service(get_all_user)
                                    .service(get_one_user)
                                    .service(register_user)
                                    .service(remove_one_user)
                                    .service(
                                        web::scope("/member")
                                            .service(user_members)
                                            .service(add_member)
                                            .service(remove_member),
                                    ),
                            ),
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
    "Authenticated".to_string()
}
