use super::trend_provider::TrendProvider;
use crate::service::strategy::trade_context_provider::TradeContextProvider;

pub trait TrendProviderFactory<T: TrendProvider> {
    fn from(trade_context_provider: TradeContextProvider) -> T;
}
