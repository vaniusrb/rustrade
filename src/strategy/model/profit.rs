use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct Profit {
    id: u64,
    symbol: String,
    position: u64,
    sold_date: DateTime<Utc>,
    sold_price: Decimal,
    sold_amount: Decimal,
    bought_date: DateTime<Utc>,
    bought_price: Decimal,
    bought_amount: Decimal,
    bought_usd: Decimal,
    sold_usd: Decimal,
    bought_order: u64,
    sold_order: u64,
    profit: Decimal,
    simulation: bool,
    percent: Decimal,
}
