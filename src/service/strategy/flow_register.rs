use super::trader_register::TradeOperation;
use crate::model::flow::Flow;
use crate::model::operation::Operation;
use crate::model::position::Position;
use crate::{
    model::side::Side, repository::repository_flow::RepositoryFlow,
    service::script::position_register::PositionRegister,
};
use colored::Colorize;
use ifmt::iformat;
use log::info;
use pad::{Alignment, PadStr};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Clone)]
pub struct FlowRegister {
    flow_repository: RepositoryFlow,
    old_real_balance_usd: Decimal,
}

impl FlowRegister {
    pub fn new(flow_repository: RepositoryFlow) -> Self {
        Self {
            flow_repository,
            old_real_balance_usd: dec!(0),
        }
    }

    pub fn set_position_old(&mut self, position: &Position) {
        self.old_real_balance_usd = position.real_balance_fiat;
    }

    pub fn set_position_new(&mut self, position: &Position, trade_operation: &TradeOperation) {
        let gain_usd = position.real_balance_fiat - self.old_real_balance_usd;
        // let _gain_usd_perc = self.old_real_balance_usd / position.real_balance_fiat;

        let (is_buyer_maker, quantity, state_str) = match trade_operation.operation {
            Operation::Buy(quantity) => (true, quantity, "Bought".pad_to_width(6).green()),
            Operation::Sell(quantity) => (false, quantity, "Sold".pad_to_width(6).red()),
        };
        let flow = Flow {
            id: 0,
            position: position.id,
            is_buyer_maker,
            time: trade_operation.now,
            price: trade_operation.price.0,
            quantity: quantity.0,
            total: quantity.0 * trade_operation.price.0,
            real_balance_fiat_old: self.old_real_balance_usd,
            real_balance_fiat_new: position.real_balance_fiat,
        };
        self.flow_repository.insert_flow(&mut flow);

        let gain_usd_str = gain_usd
            .to_string()
            .pad_to_width_with_alignment(17, Alignment::Right);
        let gain_usd_str = if gain_usd < dec!(0) {
            gain_usd_str.red()
        } else {
            gain_usd_str.green()
        };
        let message = iformat!(
            "{trade_operation.now} {state_str} \
            price {trade_operation.price} Balance USD {position.balance_asset_r()} \
            Position USD {position.real_balance_fiat_r()} \
            Gain USD {gain_usd_str}"
        );
        info!("{}", message);
    }
}
