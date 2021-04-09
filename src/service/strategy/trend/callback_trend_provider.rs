use crate::{
    model::operation::Operation,
    service::{
        script::position_register::PositionRegister,
        strategy::{
            running_script_state::RunningScriptState, trade_context_provider::TradeContextProvider,
        },
    },
};

use super::trend_provider::TrendProvider;

pub struct CallBackTrendProvider {
    call_back: Box<
        dyn Fn(PositionRegister, TradeContextProvider) -> eyre::Result<RunningScriptState>
            + Sync
            + Send
            + 'static,
    >,
}

impl CallBackTrendProvider {
    pub fn from(
        call_back: impl Fn(PositionRegister, TradeContextProvider) -> eyre::Result<RunningScriptState>
            + Sync
            + Send
            + 'static,
    ) -> Self {
        Self {
            call_back: Box::new(call_back),
        }
    }
}

impl<'a> TrendProvider for CallBackTrendProvider {
    fn trend(
        &mut self,
        position_register: &PositionRegister,
        trade_context_provider: &TradeContextProvider,
    ) -> eyre::Result<RunningScriptState> {
        // TODO remove clone
        (self.call_back)(position_register.clone(), trade_context_provider.clone())
    }
}
