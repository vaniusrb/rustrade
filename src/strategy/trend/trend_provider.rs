use crate::strategy::{trade_context_provider::TradeContextProvider, trend_enum::Trend};

pub trait TrendProvider {
    fn trend(&mut self) -> anyhow::Result<Trend>;

    fn trade_context_provider(&mut self) -> &mut TradeContextProvider;
}
