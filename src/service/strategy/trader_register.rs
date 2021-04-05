use crate::model::price::Price;
use crate::{model::operation::Operation, service::script::position_register::PositionRegister};
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
        Self {
            operation,
            now,
            price,
        }
    }
}

#[derive(Clone)]
pub struct TraderRegister {
    position_register: PositionRegister,
    trades: Vec<TradeOperation>,
}

impl TraderRegister {
    pub fn from(position_register: PositionRegister) -> Self {
        Self {
            position_register,
            trades: Vec::new(),
        }
    }

    /// Update profit from new operation
    pub fn register(&mut self, trade_operation: TradeOperation) -> eyre::Result<()> {
        self.position_register.register(&trade_operation)?;
        self.trades.push(trade_operation);
        Ok(())
    }

    pub fn position_register(&self) -> &PositionRegister {
        &self.position_register
    }
}
