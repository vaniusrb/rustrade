use crate::{
    model::operation::Operation,
    service::{
        script::position_register::PositionRegister,
        strategy::trade_context_provider::TradeContextProvider,
    },
};

use super::trend_provider::TrendProvider;

pub struct CallBackTrendProvider {
    call_back: Box<
        dyn Fn(PositionRegister, TradeContextProvider) -> eyre::Result<Option<Operation>>
            + Sync
            + Send
            + 'static,
    >,
}

impl CallBackTrendProvider {
    pub fn from(
        call_back: impl Fn(PositionRegister, TradeContextProvider) -> eyre::Result<Option<Operation>>
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
        position: &PositionRegister,
        trade_context_provider: &TradeContextProvider,
    ) -> eyre::Result<Option<Operation>> {
        (self.call_back)(*position, trade_context_provider.clone())
    }
}
