use crate::strategy::{trade_context_provider::TradeContextProvider, trend_enum::Trend};

pub trait TrendProvider {
    fn trend(&mut self, trend_context_provider: &TradeContextProvider) -> anyhow::Result<Trend>;
}
