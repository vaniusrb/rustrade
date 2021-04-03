use crate::{
    candles_utils::{datetime_to_timestamp, kline_to_candle},
    config::symbol_minutes::SymbolMinutes,
    model::candle::Candle,
};
use binance::{api::Binance, futures::market::FuturesMarket};
use chrono::{DateTime, Duration, Utc};
use eyre::{bail, Result};
use ifmt::iformat;
use log::error;
use log::{log, Level};
use std::env;

pub struct Exchange {
    api_key: String,
    secret_key: String,
    level: Level,
}

impl Exchange {
    pub fn new(level: Level) -> Result<Exchange> {
        Ok(Exchange {
            api_key: env::var("API_KEY")?,
            secret_key: env::var("SECRET_KEY")?,
            level,
        })
    }

    pub fn futures_market(&self) -> FuturesMarket {
        Binance::new(Some(self.api_key.clone()), Some(self.secret_key.clone()))
    }

    // TODO historical trades
    pub fn trades(&self) {
        let market = self.futures_market();

        // market.get_historical_trades(symbol, from_id, limit);

        // start_time: &Option<DateTime<Utc>>,
        // end_time: &Option<DateTime<Utc>>,
    }

    pub fn candles(
        &self,
        symbol_minutes: &SymbolMinutes,
        start_time: &Option<DateTime<Utc>>,
        end_time: &Option<DateTime<Utc>>,
    ) -> eyre::Result<Vec<Candle>> {
        let start_time = *start_time;
        let mut end_time = *end_time;

        if let Some(st) = start_time {
            if let Some(et) = end_time {
                if st == et {
                    end_time = Some(et + Duration::seconds(1));
                }
            }
        }
        self.internal_candles(symbol_minutes, &start_time, &end_time, 1000)
    }

    pub fn last_candle(&self, symbol_minutes: &SymbolMinutes) -> eyre::Result<Option<Candle>> {
        self.internal_candles(symbol_minutes, &None, &None, 1)
            .map(|cs| cs.last().map(|c| c.clone()))
    }

    pub fn internal_candles(
        &self,
        symbol_minutes: &SymbolMinutes,
        start_time: &Option<DateTime<Utc>>,
        end_time: &Option<DateTime<Utc>>,
        limit: u16,
    ) -> eyre::Result<Vec<Candle>> {
        let start_time = start_time.map(|d| datetime_to_timestamp(&d));
        let end_time = end_time.map(|d| datetime_to_timestamp(&d));
        let mut candles = Vec::new();

        let market = self.futures_market();

        match market.get_klines(
            symbol_minutes.symbol.to_string(),
            iformat! {"{symbol_minutes.minutes}m"},
            limit,
            start_time,
            end_time,
        ) {
            Ok(answer) => {
                match answer {
                    binance::model::KlineSummaries::AllKlineSummaries(summaries) => {
                        for summary in summaries {
                            let candle =
                                kline_to_candle(&summary, &symbol_minutes.symbol, symbol_minutes.minutes, &0u32.into());
                            log!(self.level, "{}", iformat!("{self.level:?} exchange: {candle}"));
                            candles.push(candle);
                        }
                    }
                }
                Ok(candles)
            }
            Err(e) => {
                let error = iformat!("exchange: {e}");
                error!("*** {}", error);
                for ec in e.iter() {
                    if let Some(source) = ec.source() {
                        error!("### {}", source);
                    }
                    error!("{}", ec);
                }
                bail!(error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use chrono::Duration;
    use ifmt::iprintln;

    use super::*;

    #[test]
    fn candles_test() {
        dotenv::dotenv().unwrap();
        let exchange = Exchange::new(Level::Info).unwrap();
        let start = Utc::now() - Duration::minutes(15);
        let symbol_minutes = SymbolMinutes::new("BTCUSDT", &15);
        let candles = exchange.candles(&symbol_minutes, &Some(start), &None).unwrap();
        for candle in candles {
            iprintln!("{candle}");
        }
    }

    #[test]
    fn last_candle_test() {
        dotenv::dotenv().unwrap();
        let exchange = Exchange::new(Level::Info).unwrap();
        let symbol_minutes = SymbolMinutes::new("BTCUSDT", &15);
        for i in 0..10 {
            let candle = exchange.last_candle(&symbol_minutes).unwrap();
            iprintln!("{i}: {candle:?}");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
