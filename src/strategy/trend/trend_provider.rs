use crate::strategy::{trade_context_provider::TradeContextProvider, trader_register::Position, trend_enum::Operation};

pub trait TrendProvider {
    fn trend(
        &mut self,
        position: &Position,
        trend_context_provider: &TradeContextProvider,
    ) -> eyre::Result<Option<Operation>>;
}
