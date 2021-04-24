use crate::model::position::Position;
use crate::model::quantity::Quantity;
use crate::services::trading::trade_operation::TradeOperation;
use crate::{model::operation::Operation, services::trading::flow_register::FlowRegister};
use log::warn;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;

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
                Operation::Buy(mut quantity_asset) => {
                    let mut buy_total = quantity_asset.0 * trade_operation.price.0;
                    let diff = self.position.balance_fiat - buy_total;

                    if diff < Decimal::zero() {
                        warn!("Fixing quantity to buy!");
                        quantity_asset =
                            Quantity(self.position.balance_fiat / trade_operation.price.0);
                        buy_total = quantity_asset.0 * trade_operation.price.0;
                    }

                    self.position.balance_asset += quantity_asset.0;
                    self.position.balance_fiat -= buy_total;
                }

                // I have coin and must sell to gain USD
                Operation::Sell(mut quantity_asset) => {
                    let mut sell_total = quantity_asset.0 * trade_operation.price.0;

                    let diff = self.position.balance_asset - quantity_asset.0;

                    if diff < Decimal::zero() {
                        warn!("Fixing quantity to sell!");
                        quantity_asset = Quantity(self.position.balance_asset);
                        sell_total = quantity_asset.0 * trade_operation.price.0;
                    }

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

#[cfg(test)]
mod tests {
    // use crate::repository::flow_repository::FlowRepository;

    // use super::PositionRegister;

    // fn position_register_test() {
    // TODO Create FlowRepository Trait and struct FlowRepositoryDB
    // FlowRepository
    // PositionRegister::new();
    //}
}
