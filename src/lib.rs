mod handler;
pub mod types;
pub mod routes;
mod db;

use salvo::prelude::*;
use sqlx::PgPool;
use std::sync::OnceLock;
use tera::Tera;

static TEMPLATES: OnceLock<Tera> = OnceLock::new();

pub fn init_templates() -> &'static Tera {
    TEMPLATES.get_or_init(|| {
	Tera::new("templates/**/*.html").unwrap_or_else(|e| {
	    panic!("templates init failed: {e}");
	})
    })
}

pub fn get_templates() -> &'static Tera {
    TEMPLATES.get().expect("templates not ready")
}



pub async fn init() {
    tracing_subscriber::fmt()
        .with_env_filter("debug") 
        .init();

    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&db_url).await.unwrap();
    db::STORE.set(pool).unwrap();

    init_templates();
}

pub async fn start_server() {
    let acceptor = TcpListener::new("0.0.0.0:8089").bind().await;
    Server::new(acceptor).serve(routes::route()).await;
}

