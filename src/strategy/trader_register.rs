use crate::model::{price::Price, quantity::Quantity};

use super::trend_enum::{Operation, Side};
use chrono::{DateTime, Utc};
use colored::Colorize;
use ifmt::iformat;
use log::info;
use pad::{Alignment, PadStr};
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal_macros::dec;

#[derive(Clone)]
pub struct Position {
    state: Side,
    balance_asset: Decimal,
    balance_fiat: Decimal,
    price: Price,
    real_balance_usd: Decimal,
}

impl Position {
    pub fn from_coin(balance_coin: Decimal, price: Price) -> Self {
        Self {
            state: Side::Bought,
            balance_asset: balance_coin,
            balance_fiat: dec!(0),
            price,
            real_balance_usd: balance_coin * price.0,
        }
    }
    pub fn from_usd(balance_usd: Decimal, price: Price) -> Self {
        Self {
            state: Side::Sold,
            balance_asset: dec!(0),
            balance_fiat: balance_usd,
            price,
            real_balance_usd: balance_usd,
        }
    }

    pub fn balance_coin_r(&self) -> Decimal {
        self.balance_asset
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn balance_usd_r(&self) -> Decimal {
        self.balance_fiat.round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn real_balance_usd_r(&self) -> Decimal {
        self.real_balance_usd
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn state(&self) -> &Side {
        &self.state
    }

    pub fn register(&mut self, trade_operation: &TradeOperation) {
        let old_real_balance_usd = self.real_balance_usd;

        // TODO

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

        // TODO register flow
        //          when, how much sell or bough, real usd

        self.state = trade_operation.operation.to_side();
        self.price = trade_operation.price;
        self.real_balance_usd = self.balance_asset * self.price.0 + self.balance_fiat;

        let gain_usd = self.real_balance_usd - old_real_balance_usd;
        let gain_usd_perc = old_real_balance_usd / self.real_balance_usd;

        let gain_usd_str = gain_usd.to_string().pad_to_width_with_alignment(17, Alignment::Right);

        let gain_usd_str = if gain_usd < dec!(0) {
            gain_usd_str.red()
        } else {
            gain_usd_str.green()
        };

        let state_str = self.state.to_string().pad_to_width(6);
        let state_str = match self.state {
            Side::Bought => state_str.green(),
            Side::Sold => state_str.red(),
        };

        let message = iformat!(
            "{trade_operation.now} {state_str} \
            price {trade_operation.price} Balance USD {self.balance_coin_r()} \
            Position USD {self.real_balance_usd_r()} \
            Gain USD {gain_usd_str}"
        );

        info!("{}", message);
    }
}

/// TradeOperation is a Operation with current context (date_time and price)
#[derive(Clone, Debug)]
pub struct TradeOperation {
    pub operation: Operation,
    pub now: DateTime<Utc>,
    pub price: Price,
}

impl TradeOperation {
    pub fn new(operation: Operation, now: DateTime<Utc>, price: Price) -> Self {
        Self { operation, now, price }
    }
}

#[derive(Clone)]
pub struct TraderRegister {
    position: Position,
    trades: Vec<TradeOperation>,
}

impl TraderRegister {
    pub fn from(position: Position) -> Self {
        Self {
            position,
            trades: Vec::new(),
        }
    }

    /// Update profit from new operation
    pub fn register(&mut self, trade_operation: TradeOperation) {
        self.position.register(&trade_operation);
        self.trades.push(trade_operation);
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn trades(&self) -> Vec<TradeOperation> {
        self.trades.clone()
    }
}
