use crate::strategy::{trade_context_provider::TradeContextProvider, trader_register::Position, trend_enum::Operation};

use super::trend_provider::TrendProvider;

pub struct CallBackTrendProvider {
    call_back: Box<dyn Fn(Position, TradeContextProvider) -> eyre::Result<Option<Operation>> + Sync + Send + 'static>,
}

impl CallBackTrendProvider {
    pub fn from(
        call_back: impl Fn(Position, TradeContextProvider) -> eyre::Result<Option<Operation>> + Sync + Send + 'static,
    ) -> Self {
        Self {
            call_back: Box::new(call_back),
        }
    }
}

impl<'a> TrendProvider for CallBackTrendProvider {
    fn trend(
        &mut self,
        position: &Position,
        trade_context_provider: &TradeContextProvider,
    ) -> eyre::Result<Option<Operation>> {
        (self.call_back)(position.clone(), trade_context_provider.clone())
    }
}
