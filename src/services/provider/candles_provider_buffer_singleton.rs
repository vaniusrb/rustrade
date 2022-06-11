use super::candles_buffer::CandlesBuffer;
use crate::config::candles_selection::CandlesSelection;
use crate::config::symbol_minutes::SymbolMinutes;
use crate::model::candle::Candle;
use crate::model::open_close_range::OpenCloseRange;
use crate::repository::candle_repository::CandleRepository;
use crate::services::exchange::Exchange;
use crate::services::provider::candles_range::candles_to_ranges_missing;
use crate::services::technicals::heikin_ashi;
use chrono::prelude::*;
use chrono::Duration;
use colored::Colorize;
use ifmt::iformat;
use log::debug;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;

pub struct CandlesProviderBufferSingleton {
    exchange: Exchange,
    candle_repository: CandleRepository,
    buffer: HashMap<SymbolMinutes, CandlesBuffer>,
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

        fn candles_to_buf(
            heikin_ashi: bool,
            candles: Vec<Candle>,
            candles_btree: &mut CandlesBuffer,
        ) -> eyre::Result<()> {
            if heikin_ashi {
                let candles = candles.iter().collect::<Vec<_>>();
                let candles_ref = candles.as_slice();
                let candles = heikin_ashi::heikin_ashi(candles_ref);
                candles_btree.push_candles(candles)?;
                // candles.iter().for_each(|c| {
                //     candles_btree.insert(c.open_time, *c);
                // });
            } else {
                candles_btree.push_candles(candles)?;
                // candles.iter().for_each(|c| {
                //     candles_btree.insert(c.open_time, *c);
                // });
            }
            //candles_buf.sort();
            Ok(())
        }

        // 2020-12-25 00:00
        // 2020-12-25 01:00
        // 2020-12-25 02:00
        // 2020-12-25 02:00

        // Normalize default start/end date time
        let minutes = candles_selection.symbol_minutes.minutes;
        let start_time = &candles_selection.start_time;

        // 1  % 7 + 1 = 1 * 7 = 7
        // 13 % 7 + 1 = 2 * 7 = 14

        let _end_time = (candles_selection.end_time + Duration::days(3_i64))
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        let end_time = Utc.ymd(2020, 12, 31).and_hms(23, 59, 59);

        // let end_time = candles_selection.end_time;

        let symbol_minutes = candles_selection.symbol_minutes;

        let candles_btree = loop {
            // Get candles from buffer
            debug!(
                "Retrieving candles from buffer {:?} {:?}...",
                start_time, end_time
            );

            let candles_btree = self
                .buffer
                .entry(symbol_minutes)
                .or_insert_with(|| CandlesBuffer::new(symbol_minutes.minutes));

            debug!("Candles buffer count: {}", candles_btree.len());

            // TODO
            // THE candles_to_ranges_missing IS THE BOTTLE NECK!!!
            // IF PUT THIS THE PROCESS SPEED UP!!!
            // if !candles_btree.is_empty() {
            //     break (_candles_buf, candles_btree);
            // }

            // let end_time_bt = end_time + Duration::minutes(minutes as i64);

            // let candles = candles_btree
            //     .range((Included(start_time), Included(&end_time_bt)))
            //     .map(|(_, c)| *c)
            //     .collect::<Vec<_>>();
            let ranges_missing_from_buffer = candles_btree.missing_ranges(start_time, &end_time)?;

            debug!("Retrieving ranges missing from buffer...");
            // let ranges_missing_from_buffer = candles_to_ranges_missing(
            //     &OpenCloseTime::from_date(start_time, minutes),
            //     &OpenCloseTime::from_date(&end_time, minutes),
            //     candles_selection.symbol_minutes.minutes,
            //     candles_btree.values().collect::<Vec<_>>().as_slice(),
            // )?;
            debug!(
                "Buffer ranges missing count: {}",
                ranges_missing_from_buffer.len()
            );

            if ranges_missing_from_buffer.is_empty() {
                break candles_btree;
            }

            for range_missing_from_buffer in ranges_missing_from_buffer.iter() {
                let OpenCloseRange(start_time, end_time) = range_missing_from_buffer;

                if start_time > end_time {
                    panic!("start_time {} > end_time {}", start_time, end_time);
                }

                // Get candles from repository
                debug!(
                    "{}",
                    iformat!("Retrieving candles repository {start_time:?} {end_time:?}...")
                        .to_string()
                        .red(),
                );
                let candles_repo = self
                    .candle_repository
                    .candles_by_time(
                        &candles_selection.symbol_minutes,
                        &start_time.open(minutes),
                        &end_time.open(minutes),
                    )
                    .unwrap_or_default();
                debug!("Candles repository count: {}", candles_repo.len());

                // Get ranges missing
                info!(
                    "Retrieving ranges missing from repository {}m {:?} {:?}...",
                    minutes, start_time, end_time
                );
                let ranges_missing_from_exchange = candles_to_ranges_missing(
                    &start_time,
                    &end_time,
                    candles_selection.symbol_minutes.minutes,
                    candles_repo.iter().collect::<Vec<_>>().as_slice(),
                )?;

                candles_to_buf(candles_selection.heikin_ashi, candles_repo, candles_btree)?;

                debug!(
                    "Repository ranges missing count: {}",
                    ranges_missing_from_exchange.len()
                );

                if ranges_missing_from_exchange.is_empty() {
                    break;
                }

                for range_missing_from_exchange in ranges_missing_from_exchange.iter() {
                    let OpenCloseRange(start_time, end_time) = range_missing_from_exchange;

                    info!(
                        "Retrieving candles from exchange {}m {:?} {:?}...",
                        minutes, start_time, end_time
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
                        candles_exchange,
                        candles_btree,
                    )?;
                }
            }
        };

        let candles = candles_btree
            .candles_from_range(candles_selection.start_time, candles_selection.end_time)
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        // .range((
        //     Included(candles_selection.start_time),
        //     Included(candles_selection.end_time),
        // ))
        // .map(|(_, c)| *c)
        // .collect::<Vec<_>>();

        // let candles = candles_buf
        //     .iter()
        //     .filter(|c| {
        //         c.open_time >= candles_selection.start_time
        //             && c.open_time <= candles_selection.end_time
        //     })
        //     .cloned()
        //     .collect::<Vec<_>>();
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
