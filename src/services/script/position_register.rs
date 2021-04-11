use crate::model::position::Position;
use crate::services::trader::trade_operation::TradeOperation;
use crate::{model::operation::Operation, services::trader::flow_register::FlowRegister};

#[derive(Clone)]
pub struct PositionRegister {
    pub position: Position,
    pub flow_register: FlowRegister,
}

impl PositionRegister {
    pub fn new(position: Position, flow_register: FlowRegister) -> Self {
        Self {
            position,
            flow_register,
        }
    }

    pub fn register(&mut self, trade_operation: &TradeOperation) -> eyre::Result<()> {
        self.flow_register.set_position_old(&self.position);

        {
            match trade_operation.operation {
                // I have USD and must buy coin
                Operation::Buy(quantity_asset) => {
                    let buy_total = quantity_asset.0 * trade_operation.price.0;
                    self.position.balance_asset += quantity_asset.0;
                    self.position.balance_fiat -= buy_total;
                }

                // I have coin and must sell to gain USD
                Operation::Sell(quantity_asset) => {
                    let sell_total = quantity_asset.0 * trade_operation.price.0;
                    self.position.balance_asset -= quantity_asset.0;
                    self.position.balance_fiat += sell_total;
                }
            };

            self.position.price = trade_operation.price.0;
            self.position.real_balance_fiat =
                self.position.balance_asset * self.position.price + self.position.balance_fiat;
        }

        self.flow_register
            .set_position_new(&self.position, trade_operation)?;
        Ok(())
    }
}
