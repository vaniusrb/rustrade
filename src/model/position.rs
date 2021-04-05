use super::price::Price;
use rust_decimal::Decimal;
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
    pub fn from_asset(description: String, balance_asset: Decimal, price: Price) -> Self {
        Self {
            id: 0,
            balance_asset,
            balance_fiat: dec!(0),
            price: price.0,
            real_balance_fiat: balance_asset * price.0,
            description,
        }
    }

    pub fn from_fiat(description: String, balance_fiat: Decimal) -> Self {
        Self {
            id: 0,
            balance_asset: dec!(0),
            balance_fiat,
            price: price.0,
            real_balance_fiat: balance_fiat,
            description,
        }
    }

    pub fn balance_asset_r(&self) -> Decimal {
        self.balance_asset
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn balance_fiat_r(&self) -> Decimal {
        self.balance_fiat
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn real_balance_fiat_r(&self) -> Decimal {
        self.real_balance_fiat
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }
}
