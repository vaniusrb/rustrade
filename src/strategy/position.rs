use super::operation::Operation;
use super::{flow_register::FlowRegister, side::Side};
use crate::model::price::Price;
use crate::strategy::trader_register::TradeOperation;
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal_macros::dec;

#[derive(Clone, Copy)]
pub struct Position {
    pub side: Side,
    pub balance_asset: Decimal,
    pub balance_fiat: Decimal,
    pub price: Price,
    pub real_balance_fiat: Decimal,
    pub flow_register: FlowRegister,
}

impl Position {
    pub fn _from_asset(flow_register: FlowRegister, balance_asset: Decimal, price: Price) -> Self {
        Self {
            flow_register,
            side: Side::Bought,
            balance_asset,
            balance_fiat: dec!(0),
            price,
            real_balance_fiat: balance_asset * price.0,
        }
    }

    pub fn from_fiat(flow_register: FlowRegister, balance_fiat: Decimal, price: Price) -> Self {
        Self {
            flow_register,
            side: Side::Sold,
            balance_asset: dec!(0),
            balance_fiat,
            price,
            real_balance_fiat: balance_fiat,
        }
    }

    pub fn balance_asset_r(&self) -> Decimal {
        self.balance_asset
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn balance_fiat_r(&self) -> Decimal {
        self.balance_fiat.round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn real_balance_fiat_r(&self) -> Decimal {
        self.real_balance_fiat
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn register(&mut self, trade_operation: &TradeOperation) {
        // #### self.flow_register.set_position_old(&self);
        self.flow_register.set_position_old(*self);

        {
            match trade_operation.operation {
                // I have USD and must buy coin
                Operation::Buy(quantity_asset) => {
                    let buy_total = quantity_asset.0 * trade_operation.price.0;
                    self.balance_asset += quantity_asset.0;
                    self.balance_fiat -= buy_total;
                }

                // I have coin and must sell to gain USD
                Operation::Sell(quantity_asset) => {
                    let sell_total = quantity_asset.0 * trade_operation.price.0;
                    self.balance_asset -= quantity_asset.0;
                    self.balance_fiat += sell_total;
                }
            };

            self.side = trade_operation.operation.to_side();
            self.price = trade_operation.price;
            self.real_balance_fiat = self.balance_asset * self.price.0 + self.balance_fiat;
        }

        self.flow_register.set_position_new(*self, trade_operation);
    }
}
