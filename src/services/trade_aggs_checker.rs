use super::exchange::Exchange;
use crate::model::trade_agg::TradeAgg;
use crate::repository::symbol_repository::SymbolRepository;
use crate::repository::trade_agg_repository::TradeAggRepository;
use crate::CandlesSelection;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use log::info;
use log::warn;
use log::Level;
use sqlx::PgPool;
use std::sync::Arc;
use std::sync::RwLock;

pub struct TradeAggsChecker {
    pool: Arc<RwLock<PgPool>>,
    candles_selection: CandlesSelection,
}

impl TradeAggsChecker {
    pub fn new(pool: Arc<RwLock<PgPool>>, candles_selection: CandlesSelection) -> Self {
        Self {
            pool,
            candles_selection,
        }
    }

    pub fn check(&self) -> eyre::Result<()> {
        let repository_trade_history = TradeAggRepository::new(self.pool.clone());
        let symbol = self.candles_selection.symbol_minutes.symbol;

        let end_time = repository_trade_history
            .last_trade_agg_time(symbol)
            .unwrap();
        let start_time = end_time - Duration::hours(1);

        let trades_agg = repository_trade_history.read_trades_agg_by_time(start_time, end_time)?;

        let max_min = Duration::seconds(30);

        let trades_agg_ref = trades_agg.iter().collect::<Vec<_>>();
        let missing_ranges =
            missing_ranges_trades_agg(start_time, end_time, trades_agg_ref.as_slice(), max_min)?;

        if missing_ranges.is_empty() {
            return Ok(());
        }
        warn!("Missing ranges from {} {}:", start_time, end_time);
        for missing_range in missing_ranges.iter() {
            warn!("{} {}", missing_range.0, missing_range.1);
        }

        Ok(())
    }

    /// Import last 1 hour of trades
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
                info!("No more trades from exchange");
                break;
            }

            let (to_discard, to_import): (_, Vec<TradeAgg>) = trade_histories
                .iter()
                .partition(|t| t.time <= start_time || t.id <= id_last_trade);

            let to_import: Vec<TradeAgg> = to_import.iter().copied().collect();

            info!("Inserting trades {}...", to_import.len());
            for trade in to_import.iter() {
                match repository_trade_history.insert_trade_agg(trade) {
                    Ok(_) => {}
                    Err(e) => warn!(
                        "Error {} on insert trade id {} time {}",
                        e, trade.id, trade.time
                    ),
                };
            }
            info!("Inserted trades");

            if !to_discard.is_empty() {
                info!("No trades to discard");
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
) -> eyre::Result<Vec<(DateTime<Utc>, DateTime<Utc>)>> {
    let mut last_time = start_time;
    let mut missing = Vec::new();
    for trade in trades_agg.iter() {
        if trade.time - last_time >= max_min {
            missing.push((last_time, trade.time));
        }
        last_time = trade.time;
    }
    if end_time - last_time >= max_min {
        missing.push((last_time, end_time));
    }
    Ok(missing)
}

#[cfg(test)]
mod tests {
    use crate::model::trade_agg::TradeAgg;
    use crate::services::trade_aggs_checker::missing_ranges_trades_agg;
    use crate::utils::date_utils::str_to_datetime;
    use crate::utils::dec_utils::fdec;
    use chrono::DateTime;
    use chrono::Duration;
    use chrono::Utc;
    use pretty_assertions::assert_eq;

    #[test]
    fn missing_ranges_trades_agg_test() -> color_eyre::eyre::Result<()> {
        let trades_agg = vec![
            TradeAgg::new(
                1,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:00:00"),
            ),
            TradeAgg::new(
                2,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:00:01"),
            ),
            TradeAgg::new(
                3,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:00:03"),
            ),
            TradeAgg::new(
                4,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:05:00"),
            ),
            TradeAgg::new(
                5,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:05:10"),
            ),
            TradeAgg::new(
                6,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 11:00:00"),
            ),
        ];
        let expected_missing_ranges = vec![
            (
                str_to_datetime("2021-04-11 09:50:00"),
                str_to_datetime("2021-04-11 10:00:00"),
            ),
            (
                str_to_datetime("2021-04-11 10:00:03"),
                str_to_datetime("2021-04-11 10:05:00"),
            ),
            (
                str_to_datetime("2021-04-11 10:05:10"),
                str_to_datetime("2021-04-11 11:00:00"),
            ),
            (
                str_to_datetime("2021-04-11 11:00:00"),
                str_to_datetime("2021-04-11 11:05:00"),
            ),
        ];

        let start_time = str_to_datetime("2021-04-11 09:50:00");
        let end_time = str_to_datetime("2021-04-11 11:05:00");
        let max_min = Duration::seconds(30);
        let trades_agg_ref = trades_agg.iter().collect::<Vec<_>>();
        let missing_ranges =
            missing_ranges_trades_agg(start_time, end_time, trades_agg_ref.as_slice(), max_min)?;

        assert_eq!(missing_ranges, expected_missing_ranges);

        Ok(())
    }

    #[test]
    fn no_missing_ranges_trades_agg_test() -> color_eyre::eyre::Result<()> {
        let trades_agg = vec![
            TradeAgg::new(
                1,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:00:00"),
            ),
            TradeAgg::new(
                2,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:00:20"),
            ),
            TradeAgg::new(
                3,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:00:40"),
            ),
            TradeAgg::new(
                4,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:01:00"),
            ),
            TradeAgg::new(
                5,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:01:20"),
            ),
            TradeAgg::new(
                6,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:01:40"),
            ),
            TradeAgg::new(
                7,
                1,
                fdec(0.5),
                fdec(55000.),
                str_to_datetime("2021-04-11 10:02:00"),
            ),
        ];
        let expected_missing_ranges = Vec::<(DateTime<Utc>, DateTime<Utc>)>::new();

        let start_time = str_to_datetime("2021-04-11 10:00:00");
        let end_time = str_to_datetime("2021-04-11 10:02:00");
        let max_min = Duration::seconds(30);
        let trades_agg_ref = trades_agg.iter().collect::<Vec<_>>();
        let missing_ranges =
            missing_ranges_trades_agg(start_time, end_time, trades_agg_ref.as_slice(), max_min)?;

        assert_eq!(missing_ranges, expected_missing_ranges);

        Ok(())
    }
}
