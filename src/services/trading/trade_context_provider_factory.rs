use super::trade_context_provider::TradeContextProvider;

pub trait TradeContextProviderFactory {
    fn trade_context_provider(&self) -> TradeContextProvider;
}
