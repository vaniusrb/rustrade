use crate::strategy::{operation::Operation, position::Position, trade_context_provider::TradeContextProvider};

pub trait TrendProvider {
    fn trend(
        &mut self,
        position: &Position,
        trend_context_provider: &TradeContextProvider,
    ) -> eyre::Result<Option<Operation>>;
}
