use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;

use crate::model::candle::Candle;

pub fn min_max_candles(candles: &[&Candle]) -> (f64, f64) {
    let max = candles.iter().fold(dec!(0), |acc, t| acc.max(t.high));
    let min = candles.iter().fold(max, |acc, t| acc.min(t.low));
    (min.to_f64().unwrap(), max.to_f64().unwrap())
}
