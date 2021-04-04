use super::open_close::OpenClose;
use crate::candles_utils::{str_to_datetime, time_to_str};
use chrono::{DateTime, Utc};
use ifmt::iwrite;
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct Candle {
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub id: i32,
    pub symbol: i32,
    pub minutes: Decimal,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

impl Candle {
    pub fn new(
        id: u32,
        open_time: &str,
        close_time: &str,
        symbol: i32,
        minutes: u32,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
    ) -> Self {
        Self {
            id,
            open_time: str_to_datetime(open_time),
            close_time: str_to_datetime(close_time),
            symbol, // #### symbol_from_string
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
            "{&self.symbol} [{self.minutes} {self.open_time} {close_time_fmt}] {self.close}"
        )
    }
}
