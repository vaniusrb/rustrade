use super::trend_enum::{Operation, Trend};
use chrono::{DateTime, Utc};
use colored::Colorize;
use ifmt::iformat;
use log::info;
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal_macros::dec;

#[derive(Clone)]
pub struct Position {
    state: Trend,
    balance_coin: Decimal,
    balance_usd: Decimal,
    price: Decimal,
    real_balance_usd: Decimal,
}

impl Position {
    pub fn from_coin(balance_coin: Decimal, price: Decimal) -> Self {
        Self {
            state: Trend::Bought,
            balance_coin,
            balance_usd: dec!(0),
            price,
            real_balance_usd: balance_coin * price,
        }
    }
    pub fn from_usd(balance_usd: Decimal, price: Decimal) -> Self {
        Self {
            state: Trend::Sold,
            balance_coin: dec!(0),
            balance_usd,
            price,
            real_balance_usd: balance_usd,
        }
    }

    pub fn balance_coin_r(&self) -> Decimal {
        self.balance_coin.round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn balance_usd_r(&self) -> Decimal {
        self.balance_usd.round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn real_balance_usd_r(&self) -> Decimal {
        self.real_balance_usd
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown)
    }

    pub fn state(&self) -> &Trend {
        &self.state
    }
}

#[derive(Clone, Debug)]
pub struct TradeOperation {
    pub operation: Operation,
    pub now: DateTime<Utc>,
    pub price: Decimal,
}

impl TradeOperation {
    pub fn new(operation: Operation, now: DateTime<Utc>, price: Decimal) -> Self {
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
        let old_real_balance_usd = self.position.real_balance_usd;

        match trade_operation.operation {
            // I have USD and must buy coin
            Operation::Buy => {
                let quantity_usd = self.position.balance_usd;
                let quantity_coin = quantity_usd / trade_operation.price;

                self.position.balance_coin += quantity_coin;
                self.position.balance_usd -= quantity_usd;
            }
            // I have coin and must sell to gain USD
            Operation::Sell => {
                let quantity_coin = self.position.balance_coin;
                let quantity_usd = quantity_coin * trade_operation.price;

                self.position.balance_coin -= quantity_coin;
                self.position.balance_usd += quantity_usd;
            }
        };

        // TODO register flow
        //          when, how much sell or bough, real usd

        // Update balances
        self.position.balance_coin = self
            .position
            .balance_coin
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown);
        self.position.balance_usd = self
            .position
            .balance_usd
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown);

        self.position.state = trade_operation.operation.to_trend();
        self.position.price = trade_operation.price;
        self.position.real_balance_usd = self.position.balance_coin * self.position.price + self.position.balance_usd;

        let gain_usd = self.position.real_balance_usd - old_real_balance_usd;
        let gain_usd_perc = old_real_balance_usd / self.position.real_balance_usd;

        let gain_usd_str = if gain_usd < dec!(0) {
            gain_usd.to_string().red()
        } else {
            gain_usd.to_string().green()
        };

        let message = iformat!(
            "{trade_operation.now:32} {self.position.state} \
            price {trade_operation.price} Balance USD {self.position.balance_coin_r()} \
            Position USD {self.position.real_balance_usd_r()} \
            Gain USD {gain_usd_str}"
        );

        info!("{}", message);

        self.trades.push(trade_operation);
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn trades(&self) -> Vec<TradeOperation> {
        self.trades.clone()
    }
}
