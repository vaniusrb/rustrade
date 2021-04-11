use crate::services::{
    script::position_register::PositionRegister,
    trader::{running_script_state::TrendState, trade_context_provider::TradeContextProvider},
};

use super::trend_provider::TrendProvider;

pub struct CallBackTrendProvider {
    call_back: Box<
        dyn Fn(PositionRegister, TradeContextProvider) -> eyre::Result<TrendState>
            + Sync
            + Send
            + 'static,
    >,
}

impl CallBackTrendProvider {
    pub fn from(
        call_back: impl Fn(PositionRegister, TradeContextProvider) -> eyre::Result<TrendState>
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
    ) -> eyre::Result<TrendState> {
        // TODO remove clone
        (self.call_back)(position_register.clone(), trade_context_provider.clone())
    }
}
