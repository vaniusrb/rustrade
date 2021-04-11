use crate::utils::date_utils::time_to_str;
use chrono::{DateTime, Utc};
use ifmt::iwrite;
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(sqlx::FromRow, Clone, Copy)]
pub struct TradeAgg {
    pub id: i64,
    pub symbol: i32,
    pub quantity: Decimal,
    pub price: Decimal,
    pub time: DateTime<Utc>,
}

impl TradeAgg {
    pub fn new(
        id: i64,
        symbol: i32,
        quantity: Decimal,
        price: Decimal,
        time: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            symbol,
            quantity,
            price,
            time,
        }
    }
}

impl Display for TradeAgg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time_fmt = time_to_str(&self.time);
        iwrite!(f, "[{self.id}] {time_fmt} {self.quantity} USD {self.price}")
    }
}
