use crate::strategy::{trade_context_provider::TradeContextProvider, trend_enum::Trend};

use super::trend_provider::TrendProvider;

pub struct CallBackTrendProvider {
    call_back: Box<dyn Fn(TradeContextProvider) -> anyhow::Result<Trend> + Sync + Send + 'static>,
}

impl CallBackTrendProvider {
    pub fn from(call_back: impl Fn(TradeContextProvider) -> anyhow::Result<Trend> + Sync + Send + 'static) -> Self {
        Self {
            call_back: Box::new(call_back),
        }
    }
}

impl<'a> TrendProvider for CallBackTrendProvider {
    fn trend(&mut self, trade_context_provider: &TradeContextProvider) -> anyhow::Result<Trend> {
        (self.call_back)(trade_context_provider.clone())
    }
}
