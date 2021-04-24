use crate::config::candles_selection::CandlesSelection;
use crate::config::symbol_minutes::SymbolMinutes;
use crate::model::candle::Candle;
use crate::model::open_close_time::OpenCloseTime;
use crate::repository::candle_repository::CandleRepository;
use crate::services::exchange::Exchange;
use crate::services::provider::candles_range::candles_to_ranges_missing;
use crate::services::technicals::heikin_ashi;
use ifmt::iformat;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;

pub struct CandlesProviderBufferSingleton {
    exchange: Exchange,
    candle_repository: CandleRepository,
    buffer: HashMap<SymbolMinutes, Vec<Candle>>,
}

impl CandlesProviderBufferSingleton {
    pub fn new(repository: CandleRepository, exchange: Exchange) -> Arc<RwLock<Self>> {
        let candles_provider_singleton = Self {
            exchange,
            candle_repository: repository,
            buffer: HashMap::new(),
        };
        Arc::new(RwLock::new(candles_provider_singleton))
    }

    pub fn candles(&mut self, candles_selection: CandlesSelection) -> eyre::Result<Vec<Candle>> {
        let start = Instant::now();
        debug!("Initializing import...");

        fn candles_to_buf(heikin_ashi: bool, candles: &mut Vec<Candle>, buff: &mut Vec<Candle>) {
            if heikin_ashi {
                let candles = candles.iter().collect::<Vec<_>>();
                let candles_ref = candles.as_slice();
                let mut candles = heikin_ashi::heikin_ashi(candles_ref);
                buff.append(&mut candles);
            } else {
                buff.append(candles);
            }
            buff.sort();
        }

        // Normalize default start/end date time
        let start_time = &candles_selection.start_time;
        let end_time = &candles_selection.end_time;
        let minutes = candles_selection.symbol_minutes.minutes;
        let symbol_minutes = candles_selection.symbol_minutes;

        let candles_buf = loop {
            // Get candles from buffer
            debug!(
                "Retrieving candles buffer {:?} {:?}...",
                start_time, end_time
            );
            let mut candles_buf = self.buffer.entry(symbol_minutes).or_default();
            debug!("Candles buffer count: {}", candles_buf.len());

            debug!("Retrieving ranges missing from buffer...");
            let ranges_missing_from_buffer = candles_to_ranges_missing(
                &OpenCloseTime::from_date(start_time, minutes),
                &OpenCloseTime::from_date(end_time, minutes),
                candles_selection.symbol_minutes.minutes,
                candles_buf.iter().collect::<Vec<_>>().as_slice(),
            )?;
            debug!(
                "Buffer ranges missing count: {}",
                ranges_missing_from_buffer.len()
            );

            if ranges_missing_from_buffer.is_empty() {
                break candles_buf;
            }

            for range_missing_from_buffer in ranges_missing_from_buffer.iter() {
                let (start_time, end_time) = range_missing_from_buffer;

                // Get candles from repository
                debug!(
                    "Retrieving candles repository {:?} {:?}...",
                    start_time, end_time
                );
                let mut candles_repo = self
                    .candle_repository
                    .candles_by_time(
                        &candles_selection.symbol_minutes,
                        &start_time.open(minutes),
                        &end_time.open(minutes),
                    )
                    .unwrap_or_default();
                debug!("Candles repository count: {}", candles_repo.len());

                candles_to_buf(
                    candles_selection.heikin_ashi,
                    &mut candles_repo,
                    &mut candles_buf,
                );

                // Get ranges missing
                debug!(
                    "Retrieving ranges missing from repository {:?} {:?}...",
                    start_time, end_time
                );
                let ranges_missing_from_exchange = candles_to_ranges_missing(
                    &start_time,
                    &end_time,
                    candles_selection.symbol_minutes.minutes,
                    candles_repo.iter().collect::<Vec<_>>().as_slice(),
                )?;
                debug!(
                    "Repository ranges missing count: {}",
                    ranges_missing_from_exchange.len()
                );
                if ranges_missing_from_exchange.is_empty() {
                    break;
                }

                for range_missing_from_exchange in ranges_missing_from_exchange.iter() {
                    let (start_time, end_time) = range_missing_from_exchange;

                    debug!(
                        "Retrieving candles from exchange {:?} {:?}...",
                        start_time, end_time
                    );
                    let mut candles_exchange = self.exchange.candles(
                        &candles_selection.symbol_minutes,
                        &Some(start_time.open(minutes)),
                        &Some(end_time.open(minutes)),
                    )?;
                    debug!("Candles exchange count: {}", candles_exchange.len());

                    // Save news candles on repository
                    self.candle_repository
                        .insert_candles(&mut candles_exchange)?;

                    // Insert candles on buffer
                    candles_to_buf(
                        candles_selection.heikin_ashi,
                        &mut candles_exchange,
                        &mut candles_buf,
                    );
                }
            }
        };
        let candles = candles_buf
            .iter()
            .filter(|c| &c.open_time >= start_time && &c.open_time <= end_time)
            .cloned()
            .collect::<Vec<_>>();
        debug!(
            "{}",
            iformat!(
                "Finished candles retrieve count: {candles.len()} elapsed: {start.elapsed():?}"
            )
        );

        Ok(candles)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::repository::pool_factory::create_pool;
    use crate::repository::symbol_repository::SymbolRepository;
    use crate::str_to_datetime;
    use crate::utils;
    use eyre::Result;
    use log::{Level, LevelFilter};

    #[test]
    fn candles_provider_buffer_singleton_test() -> Result<()> {
        utils::log_utils::setup_log(LevelFilter::Debug, module_path!());

        dotenv::dotenv()?;
        let pool = create_pool(log::LevelFilter::Debug).unwrap();

        let repository_symbol = SymbolRepository::new(pool.clone());

        let exchange: Exchange = Exchange::new(repository_symbol, Level::Debug)?;
        let repository: CandleRepository = CandleRepository::new(pool);

        repository.delete_all_candles()?;

        let candles_provider_buffer_singleton_arc =
            CandlesProviderBufferSingleton::new(repository, exchange);

        let mut candles_provider_buffer_singleton =
            candles_provider_buffer_singleton_arc.write().unwrap();

        {
            let candles_selection = CandlesSelection::from(
                1,
                15,
                str_to_datetime("2020-11-11 10:00:00"),
                str_to_datetime("2020-11-11 10:30:00"),
            );
            let candles = candles_provider_buffer_singleton.candles(candles_selection);
            assert!(candles.is_ok());
            assert_eq!(candles.unwrap().len(), 3);
        }

        {
            let candles_selection = CandlesSelection::from(
                1,
                15,
                str_to_datetime("2020-11-11 11:00:00"),
                str_to_datetime("2020-11-11 11:30:00"),
            );
            let candles = candles_provider_buffer_singleton.candles(candles_selection);
            assert!(candles.is_ok());
            assert_eq!(candles.unwrap().len(), 3);
        }

        {
            let candles_selection = CandlesSelection::from(
                1,
                15,
                str_to_datetime("2020-11-11 10:00:00"),
                str_to_datetime("2020-11-11 11:30:00"),
            );
            let candles = candles_provider_buffer_singleton.candles(candles_selection);
            assert!(candles.is_ok());
            assert_eq!(candles.unwrap().len(), 7);
        }

        Ok(())
    }
}
