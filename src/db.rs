use std::sync::OnceLock;
use sqlx::PgPool;
use crate::PG_POOL;

//pub static STORE: OnceLock<PgPool> = OnceLock::new();

#[inline]
pub fn get_pgpool() -> &'static PgPool {
    //STORE.get().unwrap()
    &PG_POOL
}
