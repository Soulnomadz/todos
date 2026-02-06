mod handler;
pub mod types;
pub mod routes;
mod error;

use salvo::prelude::*;
use sqlx::PgPool;
use std::sync::OnceLock;
use tera::Tera;
use lazy_static::lazy_static;
use error::TodoError;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        Tera::new("templates/**/*.html").expect("tera template error")
    };

    pub static ref PG_POOL: PgPool = {
    	let db_url = std::env::var("DATABASE_URL")
    	    .unwrap_or_else(|e| panic!("Failed to load env var: {}", e));

    	PgPool::connect_lazy(&db_url)
    	    .unwrap_or_else(|e| panic!("Failed to create db pool: {}", e))
    };
}

#[inline]
pub fn get_pgpool() -> &'static PgPool {
    &PG_POOL
}

pub fn init() {
    init_log();
    init_env();
}

fn init_log() {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();
}

fn init_env() {
    dotenvy::dotenv().ok();
}

pub async fn start_server() {
    let acceptor = TcpListener::new("0.0.0.0:8089").bind().await;
    Server::new(acceptor).serve(routes::route()).await;
}

