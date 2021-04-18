use super::low_high_price::LowHighPrice;
use super::open_close_price::OpenClosePrice;
use super::open_close_time::OpenCloseTime;
use crate::utils::date_utils::time_to_str;
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
    pub minutes: i32,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

impl Candle {
    pub fn new(
        id: i32,
        symbol: i32,
        open_close_time: OpenCloseTime,
        minutes: i32,
        open_close_price: OpenClosePrice,
        low_high_price: LowHighPrice,
        volume: Decimal,
    ) -> Self {
        Self {
            id,
            open_time: open_close_time.open(minutes),
            close_time: open_close_time.close(minutes),
            symbol,
            minutes,
            open: open_close_price.0,
            high: open_close_price.1,
            low: low_high_price.0,
            close: low_high_price.1,
            volume,
        }
    }

    pub fn open_close(&self) -> OpenCloseTime {
        OpenCloseTime::OpenClose(self.open_time, self.close_time)
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
