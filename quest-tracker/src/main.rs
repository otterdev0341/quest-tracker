use std::sync::Arc;

use quest_tracker::{config::{self, config_loader}, infrastructure::{axum_http::http_serv::start, postgres::postgres_connection}};
use tracing::{error, info};



#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let dotenv_env = match config_loader::load() {
        Ok(env) => env,
        Err(err) => {
            error!("Failed to load ENV: {}", err);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded");

    let _postgres_pool = match postgres_connection::establish_connection(&dotenv_env.database.url) {
        Ok(pool) => pool,
        Err(error) => {
            error!("Failed to Establish Connection: {}", error);
            std::process::exit(1);
        }
    };

    info!("Postgres connection has been established");

    start(Arc::new(dotenv_env), Arc::new(_postgres_pool))
        .await
        .expect("Fail to start server");


}
