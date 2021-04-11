use crate::model::trade_agg::TradeAgg;
use chrono::DateTime;
use chrono::Utc;
use colored::Colorize;
use eyre::bail;
use ifmt::iformat;
use log::{error, info};
use sqlx::PgPool;
use std::sync::{Arc, RwLock};

pub struct TradeAggRepository {
    pool: Arc<RwLock<PgPool>>,
}

impl TradeAggRepository {
    pub fn new(pool: Arc<RwLock<PgPool>>) -> Self {
        Self { pool }
    }

    pub fn last_trade_agg_id(&self, symbol: i32) -> i64 {
        let pool = self.pool.read().unwrap();
        let future = sqlx::query_scalar!("SELECT MAX(id) FROM trade_agg WHERE symbol = $1", symbol)
            .fetch_one(&*pool);
        let result: Option<i64> = async_std::task::block_on(future).unwrap();
        result.unwrap_or_default()
    }

    pub fn read_trade_agg_by_id(&self, id: i64) -> eyre::Result<Option<TradeAgg>> {
        let pool = self.pool.read().unwrap();
        let future = sqlx::query_as!(TradeAgg, "SELECT * FROM trade_agg WHERE id = $1", id)
            .fetch_optional(&*pool);
        let result = async_std::task::block_on(future)?;
        Ok(result)
    }

    pub fn read_trades_agg_by_time(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> eyre::Result<Vec<TradeAgg>> {
        let pool = self.pool.read().unwrap();
        let future = sqlx::query_as!(
            TradeAgg,
            "SELECT * FROM trade_agg WHERE time BETWEEN $1 AND $2 ORDER BY time",
            start_time,
            end_time
        )
        .fetch_all(&*pool);
        let result = async_std::task::block_on(future)?;
        Ok(result)
    }

    /// Insert trades
    pub fn insert_trades_agg(&self, trades: &[TradeAgg]) -> eyre::Result<()> {
        // Insert trade calling method insert_trade, that returns Result<id>
        // It's convenient collect the errors for raising the error bellow with details
        let trades_errors = trades
            .iter()
            .map(|c| (c, self.insert_trade_agg(c)))
            .filter_map(|cr| match cr.1 {
                Ok(_) => None,
                Err(e) => Some((cr.0, e)),
            })
            .collect::<Vec<_>>();

        if !trades_errors.is_empty() {
            let c = trades_errors.get(0).unwrap().0;
            let e = &trades_errors.get(0).unwrap().1;
            let context = e.root_cause().to_string().red();
            let context_details = e.root_cause();
            error!("{}", iformat!("Trades add error: {trades_errors.len()}"));
            error!("{}", iformat!("First trade: {c}"));
            error!("{}", iformat!("First error: {context}"));
            error!("{}", iformat!("Details error: {context_details:?}"));

            bail!("Trades insert error! {}", context);
        }

        Ok(())
    }

    pub fn insert_trade_agg(&self, trade: &TradeAgg) -> eyre::Result<i64> {
        info!("Inserting trade {}", trade);
        let pool = self.pool.read().unwrap();
        let future = sqlx::query!(
            "INSERT INTO trade_agg ( \
                id, \
                symbol, \
                quantity, \
                time, \
                price ) \
            VALUES ( $1, $2, $3, $4, $5 ) \
            RETURNING id \
            ",
            trade.id,
            trade.symbol,
            trade.quantity,
            trade.time,
            trade.price,
        )
        .fetch_one(&*pool);
        let rec = async_std::task::block_on(future)?;
        info!("Inserted trade");
        Ok(rec.id)
    }
}
