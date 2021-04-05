use crate::repository::repository_candle::RepositoryCandle;
use crate::Exchange;
use crate::{candles_utils::inconsistent_candles, config::candles_selection::CandlesSelection};
use chrono::{Duration, Utc};
use eyre::bail;
use ifmt::iformat;
use log::{error, info};
use sqlx::PgPool;
use std::{
    sync::{Arc, RwLock},
    time::Instant,
};

pub struct Checker {
    repo: RepositoryCandle,
    exchange: Exchange,
    candles_selection: CandlesSelection,
    pool: Arc<RwLock<PgPool>>,
}

impl Checker {
    pub fn new(
        pool: Arc<RwLock<PgPool>>,
        candles_selection: CandlesSelection,
        repository: RepositoryCandle,
        exchange: Exchange,
    ) -> Self {
        Checker {
            pool,
            repo: repository,
            exchange,
            candles_selection,
        }
    }

    pub fn synchronize(&self) -> eyre::Result<()> {
        loop {
            self.repo
                .delete_last_candle(&self.candles_selection.symbol_minutes);

            let mut last_close_time = self
                .repo
                .last_candle_close_time(&self.candles_selection.symbol_minutes);

            // If not found last candle then assume last 180 days
            let last_close_time =
                last_close_time.get_or_insert_with(|| Utc::now() - Duration::days(180));

            info!("{}", iformat!("Last close time: {last_close_time:?}"));

            let mut candles = self.exchange.candles(
                &self.candles_selection.symbol_minutes,
                &Some(*last_close_time),
                &None,
            )?;

            let mut last_id = self.repo.last_candle_id();

            // Assign id to new candles
            candles.iter_mut().for_each(|c| {
                c.id = {
                    last_id += 1;
                    last_id
                }
            });

            // ### This is a cool example to demonstrate error handling, where error is a type, it's possible capture, filter and transform (adding some semi-context with tuples)
            // Insert candles on repository with `add_candle` and filter by errors
            let candles_errors = candles
                .iter()
                .map(|c| (c, self.repo.insert_candle(c)))
                .filter(|cr| cr.1.is_err())
                .collect::<Vec<_>>();
            if !candles_errors.is_empty() {
                error!("{}", iformat!("Candles add error: {candles_errors.len()}"));
                bail!("Candles add error");
            }

            info!("{}", iformat!("Imported candles: {candles.len()}"));
            if candles.is_empty() {
                break;
            }
        }
        Ok(())
    }

    pub fn check_inconsist(&self) {
        let start = Instant::now();
        let start_time = self.candles_selection.start_time;
        let end_time = self.candles_selection.end_time;
        info!(
            "{}",
            iformat!("Check consistent: {self.candles_selection.symbol_minutes:?} {start_time:?} {end_time:?}")
        );

        let candles = self
            .repo
            .candles_by_time(
                &self.candles_selection.symbol_minutes,
                &start_time,
                &end_time,
            )
            .unwrap_or_default();

        info!("{}", iformat!("Found candles: {candles.len()}"));

        let candles_ref: Vec<_> = candles.iter().collect();

        let inconsist = inconsistent_candles(
            candles_ref.as_slice(),
            &Duration::minutes(self.candles_selection.symbol_minutes.minutes as i64),
        );
        info!("{}", iformat!("Inconsist candles: {inconsist.len()}"));
        for candle in inconsist.iter() {
            info!("{}", iformat!("{candle}"));
        }
        info!("{}", iformat!("Elapsed: {start.elapsed():?}"));
    }

    pub fn delete_inconsist(&self) {
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(180);
        let repo = RepositoryCandle::new(self.pool.clone());

        let candles = repo
            .candles_by_time(
                &self.candles_selection.symbol_minutes,
                &start_time,
                &end_time,
            )
            .unwrap_or_default();

        info!("{}", iformat!("Found candles: {candles.len()}"));

        let candles_ref: Vec<_> = candles.iter().collect();

        info!("Inconsist candles:");
        let inconsist = inconsistent_candles(
            candles_ref.as_slice(),
            &Duration::minutes(self.candles_selection.symbol_minutes.minutes as i64),
        );
        for candle in inconsist.iter() {
            info!("{}", iformat!("{candle}"));
            self.repo.delete_candle(candle.id);
        }
    }
}
