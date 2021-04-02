use crate::strategy::{
    trade_context_provider::TradeContextProvider,
    trend_enum::{Operation, Side},
};

pub trait TrendProvider {
    fn trend(&mut self, trend_context_provider: &TradeContextProvider) -> eyre::Result<Option<Operation>>;
}
