use crate::candles_utils::time_to_str;
use chrono::{DateTime, Utc};
use ifmt::iwrite;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::fmt::Display;

#[derive(sqlx::FromRow)]
pub struct TradeHistory {
    pub id: Decimal,
    pub symbol: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub time: DateTime<Utc>,
    pub is_buyer_maker: Decimal,
}

impl Display for TradeHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time_fmt = time_to_str(&self.time);
        let side = if self.is_buyer_maker == dec!(1) {
            "buy"
        } else {
            "sell"
        };
        iwrite!(
            f,
            "[{time_fmt} {&self.symbol}] {side} {self.quantity} {self.price}"
        )
    }
}