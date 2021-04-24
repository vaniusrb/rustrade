use crate::services::script::position_register::PositionRegister;
use crate::services::trading::{
    running_script_state::TrendState, trade_context_provider::TradeContextProvider,
};

pub trait TrendProvider {
    fn trend(
        &mut self,
        position: &PositionRegister,
        trade_context_provider: &TradeContextProvider,
    ) -> eyre::Result<TrendState>;
}
