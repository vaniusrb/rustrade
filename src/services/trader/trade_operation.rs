use crate::model::operation::Operation;
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
        Self {
            operation,
            now,
            price,
        }
    }
}
