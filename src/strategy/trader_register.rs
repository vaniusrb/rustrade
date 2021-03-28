use super::trend_enum::{Operation, Trend};
use chrono::{DateTime, Utc};
use colored::Colorize;
use log::debug;
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal_macros::dec;

pub static STATE_BOUGHT: &str = "bought";
pub static STATE_SOLD: &str = "sold";

#[derive(Clone)]
pub struct Position {
    state: Trend,
    balance_coin: Decimal,
    balance_usd: Decimal,
}

impl Position {
    pub fn from_coin(balance_coin: Decimal) -> Self {
        Self {
            state: Trend::Bought,
            balance_coin,
            balance_usd: dec!(0),
        }
    }
    pub fn from_usd(balance_usd: Decimal) -> Self {
        Self {
            state: Trend::Sold,
            balance_coin: dec!(0),
            balance_usd,
        }
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

    pub fn register(&mut self, trade_operation: TradeOperation) {
        match trade_operation.operation {
            // I have USD and must buy coin
            Operation::Buy => {
                let quantity_usd = self.position.balance_usd;
                let quantity_coin = quantity_usd / trade_operation.price;

                self.position.balance_coin += quantity_coin;
                self.position.balance_usd -= quantity_usd;
            }
            // I have USD and must buy coin
            Operation::Sell => {
                let quantity_coin = self.position.balance_coin;
                let quantity_usd = quantity_coin * trade_operation.price;

                self.position.balance_coin -= quantity_coin;
                self.position.balance_usd += quantity_usd;
            }
        };

        self.position.balance_coin = self
            .position
            .balance_coin
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown);
        self.position.balance_usd = self
            .position
            .balance_usd
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown);

        self.position.state = trade_operation.operation.to_trend();

        let message = match self.position.state {
            Trend::Bought => format!(
                "{} Bought price {} Balance USD {}",
                trade_operation.now, trade_operation.price, self.position.balance_usd
            )
            .green(),
            Trend::Sold => format!(
                "{} Sold price {} Balance USD {}",
                trade_operation.now, trade_operation.price, self.position.balance_usd
            )
            .red(),
        };
        debug!("{}", message);

        self.trades.push(trade_operation);
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn trades(&self) -> Vec<TradeOperation> {
        self.trades.clone()
    }
}
