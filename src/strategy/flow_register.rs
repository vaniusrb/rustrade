use super::{position::Position, trader_register::TradeOperation};
use crate::strategy::side::Side;
use colored::Colorize;
use ifmt::iformat;
use log::info;
use pad::{Alignment, PadStr};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Clone, Copy)]
pub struct FlowRegister {
    old_real_balance_usd: Decimal,
}

impl FlowRegister {
    pub fn new() -> Self {
        Self {
            old_real_balance_usd: dec!(0),
        }
    }

    pub fn set_position_old(&mut self, position: Position) {
        self.old_real_balance_usd = position.real_balance_fiat;
    }

    pub fn set_position_new(&mut self, position: Position, trade_operation: &TradeOperation) {
        let gain_usd = position.real_balance_fiat - self.old_real_balance_usd;
        let _gain_usd_perc = self.old_real_balance_usd / position.real_balance_fiat;

        let gain_usd_str = gain_usd.to_string().pad_to_width_with_alignment(17, Alignment::Right);

        let gain_usd_str = if gain_usd < dec!(0) {
            gain_usd_str.red()
        } else {
            gain_usd_str.green()
        };

        let state_str = position.side.to_string().pad_to_width(6);
        let state_str = match position.side {
            Side::Bought => state_str.green(),
            Side::Sold => state_str.red(),
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
