use crate::model::flow::Flow;
use sqlx::PgPool;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct FlowRepository {
    pool: Arc<RwLock<PgPool>>,
}

impl FlowRepository {
    pub fn new(pool: Arc<RwLock<PgPool>>) -> Self {
        Self { pool }
    }

    pub fn last_flow_id(&self) -> i32 {
        let pool = self.pool.read().unwrap();
        let future = sqlx::query_as("SELECT MAX(id) FROM flow").fetch_one(&*pool);
        let result: (Option<i32>,) = async_std::task::block_on(future).unwrap();
        result.0.unwrap_or_default()
    }

    pub fn delete_flow(&self, id: i32) {
        let pool = self.pool.read().unwrap();

        let future = sqlx::query!("DELETE FROM flow WHERE id = $1", id).execute(&*pool);
        async_std::task::block_on(future).unwrap();
    }

    pub fn delete_flows_from_position(&self, id_flow: i32) {
        let pool = self.pool.read().unwrap();

        let future = sqlx::query!("DELETE FROM flow WHERE position = $1", id_flow).execute(&*pool);
        async_std::task::block_on(future).unwrap();
    }

    /// Insert flow
    pub fn insert_flow(&self, flow: &mut Flow) -> eyre::Result<i32> {
        flow.id = self.last_flow_id() + 1;
        let pool = self.pool.read().unwrap();

        let future = sqlx::query!(
            "INSERT INTO flow ( \
                id, \
                position, \
                is_buyer_maker, \
                time, \
                price, \
                quantity, \
                total, \
                real_balance_fiat_old, \
                real_balance_fiat_new, \
                gain_perc,
                log
                ) \
                VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11 ) \
                RETURNING id \
            ",
            flow.id,
            flow.position,
            flow.is_buyer_maker,
            flow.time,
            flow.price,
            flow.quantity,
            flow.total,
            flow.real_balance_fiat_old,
            flow.real_balance_fiat_new,
            flow.gain_perc,
            flow.log,
        )
        .fetch_one(&*pool);
        let rec = async_std::task::block_on(future)?;
        Ok(rec.id)
    }
}
