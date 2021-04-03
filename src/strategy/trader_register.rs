use super::operation::Operation;
use super::position::Position;
use crate::model::price::Price;
use chrono::{DateTime, Utc};

/// TradeOperation is a Operation with current context (date_time and price)
#[derive(Clone, Debug, Copy)]
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

    // pub fn trades(&self) -> Vec<TradeOperation> {
    //     self.trades.clone()
    // }
}
