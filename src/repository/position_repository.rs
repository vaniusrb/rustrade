use crate::model::position::Position;
use sqlx::PgPool;
use std::sync::{Arc, RwLock};

pub struct PositionRepository {
    pool: Arc<RwLock<PgPool>>,
}

impl PositionRepository {
    pub fn new(pool: Arc<RwLock<PgPool>>) -> Self {
        Self { pool }
    }

    pub fn last_position_id(&self) -> i32 {
        let pool = self.pool.read().unwrap();
        let future = sqlx::query_as("SELECT MAX(id) FROM position").fetch_one(&*pool);
        let result: (Option<i32>,) = async_std::task::block_on(future).unwrap();
        result.0.unwrap_or_default()
    }

    pub fn position_by_description(&self, description: &str) -> Option<Position> {
        let pool = self.pool.read().unwrap();
        let future = sqlx::query_as!(
            Position,
            "SELECT * FROM position WHERE description = $1",
            description
        )
        .fetch_one(&*pool);
        async_std::task::block_on(future).ok()
    }

    pub fn delete_position(&self, id: i32) {
        let pool = self.pool.read().unwrap();

        let future = sqlx::query!("DELETE FROM position WHERE id = $1", id).execute(&*pool);
        async_std::task::block_on(future).unwrap();
    }

    /// Insert position
    pub fn insert_position(&self, position: &mut Position) -> eyre::Result<i32> {
        position.id = self.last_position_id() + 1;
        let pool = self.pool.read().unwrap();
        let future = sqlx::query!(
            "INSERT INTO position ( \
                id, \
                balance_asset, \
                balance_fiat, \
                price, \
                real_balance_fiat, \
                description \
                ) \
                VALUES ( $1, $2, $3, $4, $5, $6 ) \
                RETURNING id \
                ",
            position.id,
            position.balance_asset,
            position.balance_fiat,
            position.price,
            position.real_balance_fiat,
            position.description,
        )
        .fetch_one(&*pool);
        let rec = async_std::task::block_on(future)?;
        Ok(rec.id)
    }
}
