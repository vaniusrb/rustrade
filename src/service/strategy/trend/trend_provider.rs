use crate::model::operation::Operation;
use crate::model::position::Position;
use crate::service::strategy::trade_context_provider::TradeContextProvider;

pub trait TrendProvider {
    fn trend(
        &mut self,
        position: &Position,
        trend_context_provider: &TradeContextProvider,
    ) -> eyre::Result<Option<Operation>>;
}
