use super::trader_register::TradeOperation;
use crate::model::flow::Flow;
use crate::model::operation::Operation;
use crate::model::position::Position;
use crate::repository::repository_flow::RepositoryFlow;
use colored::Colorize;
use ifmt::iformat;
use log::info;
use pad::{Alignment, PadStr};
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
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

    pub fn set_position_new(
        &mut self,
        position: &Position,
        trade_operation: &TradeOperation,
    ) -> eyre::Result<()> {
        let gain_usd = (position.real_balance_fiat - self.old_real_balance_usd)
            .round_dp_with_strategy(8, RoundingStrategy::RoundDown);

        let gain_perc = ((position.real_balance_fiat / self.old_real_balance_usd - dec!(1))
            * dec!(100))
        .round_dp_with_strategy(2, RoundingStrategy::RoundDown);

        let (is_buyer_maker, quantity, state_str) = match trade_operation.operation {
            Operation::Buy(quantity) => (true, quantity, "Bought".pad_to_width(6).green()),
            Operation::Sell(quantity) => (false, quantity, "Sold".pad_to_width(6).red()),
        };
        let mut flow = Flow {
            id: 0,
            position: position.id,
            is_buyer_maker,
            time: trade_operation.now,
            price: trade_operation.price.0,
            quantity: quantity.0,
            total: quantity.0 * trade_operation.price.0,
            real_balance_fiat_old: self.old_real_balance_usd,
            real_balance_fiat_new: position.real_balance_fiat,
            gain_perc,
        };
        self.flow_repository.insert_flow(&mut flow)?;

        let gain_usd_str = gain_usd
            .to_string()
            .pad_to_width_with_alignment(14, Alignment::Right);
        let gain_perc_str =
            iformat!("{gain_perc}%").pad_to_width_with_alignment(6, Alignment::Right);

        let (gain_usd_str, gain_perc_str) = if gain_usd < dec!(0) {
            (gain_usd_str.red(), gain_perc_str.red())
        } else {
            (gain_usd_str.green(), gain_perc_str.green())
        };

        let real_balance_fiat_str = position
            .real_balance_fiat_r()
            .to_string()
            .pad_to_width_with_alignment(14, Alignment::Right);

        let message = iformat!(
            "{trade_operation.now} {state_str} \
            price {trade_operation.price} Balance USD {position.balance_asset_r()} \
            Position USD {real_balance_fiat_str} \
            Gain USD {gain_usd_str} {gain_perc_str}"
        );
        info!("{}", message);
        Ok(())
    }
}
