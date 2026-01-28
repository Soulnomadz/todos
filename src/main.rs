mod handler;
mod types;
mod routes;
mod db;

use handler::*;
use types::*;
use routes::*;
use db::*;


use salvo::prelude::*;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
	.with_env_filter("debug") 
	.init();

    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&db_url).await.unwrap();
    STORE.set(pool).unwrap();

    start_server().await;
}

pub(crate) async fn start_server() {
    let acceptor = TcpListener::new("0.0.0.0:8089").bind().await;
    Server::new(acceptor).serve(route()).await;
}

