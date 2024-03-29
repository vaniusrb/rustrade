use super::trade_operation::TradeOperation;
use crate::services::script::position_register::PositionRegister;

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
