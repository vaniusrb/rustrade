use crate::service::script::position_register::PositionRegister;
use crate::service::strategy::{
    running_script_state::RunningScriptState, trade_context_provider::TradeContextProvider,
};

pub trait TrendProvider {
    fn trend(
        &mut self,
        position: &PositionRegister,
        trend_context_provider: &TradeContextProvider,
    ) -> eyre::Result<RunningScriptState>;
}
