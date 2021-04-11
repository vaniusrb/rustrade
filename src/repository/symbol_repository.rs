use crate::model::symbol::Symbol;
use sqlx::PgPool;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct SymbolRepository {
    pool: Arc<RwLock<PgPool>>,
}

impl SymbolRepository {
    pub fn new(pool: Arc<RwLock<PgPool>>) -> Self {
        Self { pool }
    }

    pub fn symbol_by_id(&self, id: i32) -> Option<Symbol> {
        let pool = self.pool.read().unwrap();
        let future =
            sqlx::query_as!(Symbol, "SELECT * FROM symbol WHERE id = $1", id).fetch_one(&*pool);
        async_std::task::block_on(future).ok()
    }

    pub fn symbol_by_pair(&self, pair: &str) -> Option<Symbol> {
        let pool = self.pool.read().unwrap();
        let future = sqlx::query_as!(Symbol, "SELECT * FROM symbol WHERE symbol = $1", pair)
            .fetch_one(&*pool);
        async_std::task::block_on(future).ok()
    }
}
