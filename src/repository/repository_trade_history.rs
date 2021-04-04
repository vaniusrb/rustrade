use crate::candles_utils::time_to_str;
use crate::model::trade_history::TradeHistory;
use chrono::{DateTime, Utc};
use colored::Colorize;
use eyre::bail;
use ifmt::{iformat, iwrite};
use log::error;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::PgPool;
use std::fmt::Display;
struct RepositoryTradeHistory {
    pool: PgPool,
}

impl RepositoryTradeHistory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn last_trade_id(&self) -> Decimal {
        let future = sqlx::query_as("SELECT MAX(id) FROM trade").fetch_one(&self.pool);
        let result: (Option<Decimal>,) = async_std::task::block_on(future).unwrap();
        result.0.unwrap_or_default()
    }

    pub fn read_by_id(&self, id: Decimal) -> eyre::Result<Option<TradeHistory>> {
        let future = sqlx::query_as!(TradeHistory, "SELECT * FROM trade WHERE id = $1", id)
            .fetch_optional(&self.pool);
        Ok(async_std::task::block_on(future)?)
    }

    /// Insert trades
    pub fn insert_trades(&self, trades: &mut [TradeHistory]) -> eyre::Result<()> {
        let mut trade_id = self.last_trade_id();
        let one = dec!(1);
        trades.iter_mut().for_each(|c| {
            c.id = {
                trade_id += one;
                trade_id
            }
        });

        // Insert trade calling method insert_trade, that returns Result<id>
        // It's convenient collect the errors for raising the error bellow with details
        let trades_errors = trades
            .iter()
            .map(|c| (c, self.insert_trade(c)))
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

            bail!("Trades insert error!");
        }

        Ok(())
    }

    pub fn insert_trade(&self, trade: &TradeHistory) -> eyre::Result<Decimal> {
        let future = sqlx::query!(
            "INSERT INTO trade ( \
                id, \
                symbol, \
                quantity, \
                time, \
                is_buyer_maker ) \
            VALUES ( $1, $2, $3, $4, $5 ) \
            RETURNING id \
            ",
            trade.id,
            trade.symbol,
            trade.quantity,
            trade.time,
            trade.is_buyer_maker
        )
        .fetch_one(&self.pool);
        let rec = async_std::task::block_on(future)?;
        Ok(rec.id)
    }
}

// #[test]
// fn sqlx_test() -> eyre::Result<()> {
//     let pool = create_pool(LevelFilter::Debug)?;
//     let symbol = "BTCUSDT".to_string();
//     let minutes = 15u32;
//     let stream = sqlx::query_as::<_, Trade>("SELECT * FROM candle WHERE symbol = ? OR minutes = ?")
//         .bind(symbol)
//         .bind(minutes)
//         .fetch_all(&pool);

//     let rows: Vec<Trade> = async_std::task::block_on(stream).unwrap();

//     Ok(())
// }