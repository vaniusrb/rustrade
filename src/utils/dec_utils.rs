use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use rust_decimal_macros::dec;

pub const DEC_1: Decimal = dec!(1);
pub const DEC_100: Decimal = dec!(100);

pub fn fdec(value: f64) -> Decimal {
    Decimal::from_str(&value.to_string()).unwrap()
}

#[inline]
pub fn percent(new: &Decimal, old: &Decimal) -> Decimal {
    ((new / old - DEC_1) * DEC_100).round_dp_with_strategy(2, RoundingStrategy::ToZero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_test() {
        assert_eq!(percent(&dec!(150), &dec!(100)), dec!(50));
        assert_eq!(percent(&dec!(100), &dec!(100)), dec!(0));
        assert_eq!(percent(&dec!(90), &dec!(100)), dec!(-10));
    }
}
