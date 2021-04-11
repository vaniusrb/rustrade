use crate::services::script::position_register::PositionRegister;
use crate::services::trader::{
    running_script_state::TrendState, trade_context_provider::TradeContextProvider,
};

pub trait TrendProvider {
    fn trend(
        &mut self,
        position: &PositionRegister,
        trend_context_provider: &TradeContextProvider,
    ) -> eyre::Result<TrendState>;
}
