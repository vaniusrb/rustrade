use super::open_close::OpenClose;
use crate::candles_utils::{str_to_datetime, time_to_str};
use chrono::{DateTime, Utc};
use ifmt::iwrite;
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CandleDb {
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub id: Decimal,
    pub symbol: String,
    pub minutes: Decimal,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

pub fn candle_db_to_candle(candle_db: &CandleDb) -> Candle {
    Candle {
        open_time: candle_db.open_time,
        close_time: candle_db.close_time,
        id: candle_db.id,
        symbol: symbol_from_string(&candle_db.symbol),
        minutes: candle_db.minutes,
        open: candle_db.open,
        high: candle_db.high,
        low: candle_db.low,
        close: candle_db.close,
        volume: candle_db.volume,
    }
}

pub fn _candle_to_candle_db(candle: &Candle) -> CandleDb {
    CandleDb {
        open_time: candle.open_time,
        close_time: candle.close_time,
        id: candle.id,
        symbol: symbol_to_string(&candle.symbol),
        minutes: candle.minutes,
        open: candle.open,
        high: candle.high,
        low: candle.low,
        close: candle.close,
        volume: candle.volume,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct Candle {
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub id: Decimal,
    pub symbol: [char; 7],
    pub minutes: Decimal,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

pub fn symbol_from_string(st: &str) -> [char; 7] {
    let mut result = ['\x00'; 7];
    for (i, c) in st.chars().enumerate() {
        if i == 6 {
            break;
        }
        result[i] = c;
    }
    result
}

pub fn symbol_to_string(symbol: &[char; 7]) -> String {
    let result: String = symbol.iter().filter(|c| c != &&'\x00').collect();
    result
}

impl Candle {
    pub fn new(
        id: u32,
        open_time: &str,
        close_time: &str,
        symbol: &str,
        minutes: u32,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
    ) -> Self {
        Self {
            id: Decimal::from(id),
            open_time: str_to_datetime(open_time),
            close_time: str_to_datetime(close_time),
            symbol: symbol_from_string(symbol),
            minutes: Decimal::from(minutes),
            open,
            high,
            low,
            close,
            volume,
        }
    }

    pub fn open_close(&self) -> OpenClose {
        OpenClose::OpenClose(self.open_time, self.close_time)
    }
}

impl Display for Candle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let close_time_fmt = time_to_str(&self.close_time);
        iwrite!(
            f,
            "{symbol_to_string(&self.symbol)} [{self.minutes} {self.open_time} {close_time_fmt}] {self.close}"
        )
    }
}
