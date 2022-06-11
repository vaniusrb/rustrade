use super::price::Price;
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal_macros::dec;

#[derive(Clone)]
pub struct Position {
    pub id: i32,
    pub balance_asset: Decimal,
    pub balance_fiat: Decimal,
    pub price: Decimal,
    pub real_balance_fiat: Decimal,
    pub description: String,
}

impl Position {
    pub fn from_asset(description: &str, balance_asset: Decimal, price: Price) -> Self {
        Self {
            id: 0,
            balance_asset,
            balance_fiat: dec!(0),
            price: price.0,
            real_balance_fiat: balance_asset * price.0,
            description: description.to_string(),
        }
    }

    pub fn from_fiat(description: &str, balance_fiat: Decimal) -> Self {
        Self {
            id: 0,
            balance_asset: dec!(0),
            balance_fiat,
            price: dec!(0),
            real_balance_fiat: balance_fiat,
            description: description.to_string(),
        }
    }

    pub fn balance_asset_r(&self) -> Decimal {
        self.balance_asset
            .round_dp_with_strategy(8, RoundingStrategy::ToZero)
    }

    pub fn balance_fiat_r(&self) -> Decimal {
        self.balance_fiat
            .round_dp_with_strategy(8, RoundingStrategy::ToZero)
    }

    pub fn real_balance_fiat_r(&self) -> Decimal {
        self.real_balance_fiat
            .round_dp_with_strategy(8, RoundingStrategy::ToZero)
    }
}
