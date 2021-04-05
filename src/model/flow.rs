use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct Flow {
    pub id: i32,
    pub position: i32,
    pub is_buyer_maker: bool,
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub quantity: Decimal,
    pub total: Decimal,
    pub real_balance_fiat_old: Decimal,
    pub real_balance_fiat_new: Decimal,
}
