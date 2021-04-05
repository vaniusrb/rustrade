use crate::{
    candles_utils::{datetime_to_timestamp, kline_to_candle},
    config::symbol_minutes::SymbolMinutes,
    model::candle::Candle,
    repository::repository_symbol::RepositorySymbol,
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
    repository_symbol: RepositorySymbol,
}

impl Exchange {
    pub fn new(repository_symbol: RepositorySymbol, level: Level) -> Result<Exchange> {
        Ok(Exchange {
            api_key: env::var("API_KEY")?,
            secret_key: env::var("SECRET_KEY")?,
            level,
            repository_symbol,
        })
    }

    pub fn futures_market(&self) -> FuturesMarket {
        Binance::new(Some(self.api_key.clone()), Some(self.secret_key.clone()))
    }

    // TODO historical trades
    pub fn historical_trades(&self, symbol: i32, from_id: Option<u64>) -> eyre::Result<()> {
        let symbol = self.repository_symbol.symbol_by_id(symbol).unwrap().symbol;
        let market = self.futures_market();

        // symbol	STRING	YES
        // limit	INT	NO	Default 500; max 1000.
        // fromId	LONG	NO	TradeId to fetch from. Default gets most recent trades.

        match market.get_historical_trades(symbol, from_id, 1000) {
            Ok(trades) => {
                if let binance::futures::model::Trades::AllTrades(trades) = trades {
                    for trade in trades.iter() {
                        // trade.is_buyer_maker
                        // trade.qty
                        // trade.time
                        // trade.price
                        // trade.id
                    }
                }
            }
            Err(e) => {
                bail!("{}", e)
            }
        };
        Ok(())
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
        let symbol = self
            .repository_symbol
            .symbol_by_id(symbol_minutes.symbol)
            .unwrap()
            .symbol;
        let start_time = start_time.map(|d| datetime_to_timestamp(&d));
        let end_time = end_time.map(|d| datetime_to_timestamp(&d));
        let mut candles = Vec::new();

        let market = self.futures_market();

        match market.get_klines(
            symbol,
            iformat! {"{symbol_minutes.minutes}m"},
            limit,
            start_time,
            end_time,
        ) {
            Ok(answer) => {
                match answer {
                    binance::model::KlineSummaries::AllKlineSummaries(summaries) => {
                        for summary in summaries {
                            let candle = kline_to_candle(
                                &summary,
                                symbol_minutes.symbol,
                                symbol_minutes.minutes,
                                0i32,
                            );
                            log!(
                                self.level,
                                "{}",
                                iformat!("{self.level:?} exchange: {candle}")
                            );
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

    use super::*;
    use crate::repository::repository_factory::create_pool;
    use chrono::Duration;
    use ifmt::iprintln;
    use log::LevelFilter;

    #[test]
    fn candles_test() {
        dotenv::dotenv().unwrap();
        let pool = create_pool(LevelFilter::Debug).unwrap();
        let repository_symbol = RepositorySymbol::new(pool);

        let exchange = Exchange::new(repository_symbol, Level::Info).unwrap();
        let start = Utc::now() - Duration::minutes(15);
        let symbol_minutes = SymbolMinutes::new(1, 15);
        let candles = exchange
            .candles(&symbol_minutes, &Some(start), &None)
            .unwrap();
        for candle in candles {
            iprintln!("{candle}");
        }
    }

    #[test]
    fn last_candle_test() {
        dotenv::dotenv().unwrap();
        let pool = create_pool(LevelFilter::Debug).unwrap();
        let repository_symbol = RepositorySymbol::new(pool);

        let exchange = Exchange::new(repository_symbol, Level::Info).unwrap();
        let symbol_minutes = SymbolMinutes::new(1, 15);
        for i in 0..10 {
            let candle = exchange.last_candle(&symbol_minutes).unwrap();
            iprintln!("{i}: {candle:?}");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}