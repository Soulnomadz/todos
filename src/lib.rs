mod handler;
mod types;
mod routes;
mod db;

use salvo::prelude::*;
use sqlx::PgPool;

pub async fn init() {
    tracing_subscriber::fmt()
        .with_env_filter("debug") 
        .init();

    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&db_url).await.unwrap();
    db::STORE.set(pool).unwrap();
}

pub async fn start_server() {
    let acceptor = TcpListener::new("0.0.0.0:8089").bind().await;
    Server::new(acceptor).serve(routes::route()).await;
}

