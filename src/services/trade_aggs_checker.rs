use super::exchange::Exchange;
use crate::model::trade_agg::TradeAgg;
use crate::repository::symbol_repository::SymbolRepository;
use crate::repository::trade_agg_repository::TradeAggRepository;
use crate::CandlesSelection;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use eyre::eyre;
use log::info;
use log::Level;
use sqlx::PgPool;
use std::sync::Arc;
use std::sync::RwLock;

pub struct TradeAggsChecker {
    pool: Arc<RwLock<PgPool>>,
    repository_symbol: SymbolRepository,
    candles_selection: CandlesSelection,
}

impl TradeAggsChecker {
    pub fn new(
        pool: Arc<RwLock<PgPool>>,
        repository_symbol: SymbolRepository,
        candles_selection: CandlesSelection,
    ) -> Self {
        Self {
            pool,
            repository_symbol,
            candles_selection,
        }
    }

    pub fn check(&self) -> eyre::Result<()> {
        let repository_trade_history = TradeAggRepository::new(self.pool.clone());

        let end_time = Utc::now();
        let start_time = end_time - Duration::hours(1);

        let trades_agg = repository_trade_history.read_trades_agg_by_time(start_time, end_time)?;
        let first = match trades_agg.first() {
            Some(first) => first,
            None => return Ok(()),
        };
        let max_min = Duration::seconds(30);
        let mut last_time = first.time;
        for trade in trades_agg.iter() {
            if trade.time - last_time >= max_min {
                return Err(eyre!("Period is greater: {} - {}", trade, last_time));
            }
            last_time = trade.time;
        }
        Ok(())
    }

    pub fn import(&self) -> eyre::Result<()> {
        let repository_trade_history = TradeAggRepository::new(self.pool.clone());
        let symbol = self.candles_selection.symbol_minutes.symbol;

        let exchange: Exchange =
            Exchange::new(SymbolRepository::new(self.pool.clone()), Level::Debug)?;

        let id_last_trade = repository_trade_history.last_trade_agg_id(symbol);

        let end_time = Utc::now();
        let start_time = end_time - Duration::hours(1);

        let mut from_id = Option::<u64>::None;
        loop {
            info!("Retrieving from exchange {:?}...", from_id);
            let trade_histories = exchange.historical_trades(symbol, from_id)?;
            info!("Retrieved from exchange {} records", trade_histories.len());
            if trade_histories.is_empty() {
                break;
            }

            let (to_discard, to_import): (_, Vec<TradeAgg>) = trade_histories
                .iter()
                .partition(|t| t.time <= start_time || t.id <= id_last_trade);

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

pub fn missing_ranges_trades_agg(
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    trades_agg: &[&TradeAgg],
    max_min: Duration,
) -> eyre::Result<()> {
    let first_trade = match trades_agg.first() {
        Some(first) => first,
        None => return Ok(()),
    };
    let mut missing = Vec::new();

    let mut start_range_opt = Option::<DateTime<Utc>>::None;

    let max_min = Duration::seconds(30);
    let mut last_time = first_trade.time;

    for trade in trades_agg.iter() {
        if trade.time - last_time >= max_min {
            if let Some(start_range) = start_range_opt {
                missing.push((start_range + max_min, trade.time - max_min))
            }
            start_range_opt = Some(trade.time);
        }
        last_time = trade.time;
    }
    if let Some(start_range) = start_range_opt {
        missing.push((start_range + max_min, trade.time - max_min))
    }
    Ok(())
}
