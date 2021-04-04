use crate::service::strategy::trade_context_provider::TradeContextProvider;
use crate::{model::operation::Operation, service::script::position_register::PositionRegister};

pub trait TrendProvider {
    fn trend(
        &mut self,
        position: &PositionRegister,
        trend_context_provider: &TradeContextProvider,
    ) -> eyre::Result<Option<Operation>>;
}
