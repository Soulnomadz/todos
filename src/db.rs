use std::sync::OnceLock;
use sqlx::PgPool;

pub static STORE: OnceLock<PgPool> = OnceLock::new();

#[inline]
pub fn get_pgpool() -> &'static PgPool {
    STORE.get().unwrap()
}
