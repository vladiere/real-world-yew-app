use tracing_subscriber::EnvFilter;

mod dev_db;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    dev_db::init_dev().await;
}
