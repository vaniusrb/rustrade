use std::sync::Arc;
use std::sync::RwLock;

use crate::{
    model::trade_agg::TradeAgg, repository::trade_agg_repository::TradeAggRepository,
    services::exchange::Exchange,
};
use chrono::{Duration, Utc};
use sqlx::Pool;
use sqlx::Postgres;

pub struct TradeHistoryProvider {
    pool: Arc<RwLock<Pool<Postgres>>>,
    exchange: Exchange,
}

impl TradeHistoryProvider {
    pub fn new(pool: Arc<RwLock<Pool<Postgres>>>, exchange: Exchange) -> Self {
        Self { pool, exchange }
    }

    pub fn sync(&self) -> eyre::Result<()> {
        let repository_trade_history = TradeAggRepository::new(self.pool.clone());
        let symbol = 1;

        let id_last_trade = repository_trade_history.last_trade_agg_id(symbol);

        let now = Utc::now();
        let start = now - Duration::hours(1);

        let mut from_id = Option::<u64>::None;
        loop {
            let trade_histories = self.exchange.historical_trades(symbol, from_id)?;

            let (to_discard, to_import): (_, Vec<TradeAgg>) = trade_histories
                .iter()
                .partition(|t| t.time <= start || t.id <= id_last_trade);

            let to_import: Vec<TradeAgg> = to_import.iter().copied().collect();

            repository_trade_history.insert_trades_agg(&to_import)?;
            if !to_discard.is_empty() {
                break;
            }
            from_id = to_import
                .iter()
                .min_by(|t1, t2| t1.id.cmp(&t2.id))
                .map(|t| t.id as u64)
                .or(from_id);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    //#[test]
    // fn trade_history_test() -> color_eyre::eyre::Result<()> {
    //     dotenv::dotenv().unwrap();
    // }
}
