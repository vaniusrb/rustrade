use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub const DEC_1: Decimal = dec!(1);
pub const DEC_100: Decimal = dec!(100);

pub fn fdec(value: f64) -> Decimal {
    Decimal::from_str(&value.to_string()).unwrap()
}
