use super::{
    trader::Trader, trader_register::TraderRegister, trend::trend_provider::TrendProvider,
};
use crate::config::candles_selection::CandlesSelection;
use crate::services::provider::candles_provider_buffer::CandlesProviderBuffer;
use crate::services::technicals::ind_provider::IndicatorProvider;

#[derive(Clone)]
pub struct TraderFactory {
    candles_selection: CandlesSelection,
    candles_provider: CandlesProviderBuffer,
}

impl TraderFactory {
    pub fn from(
        candles_selection: CandlesSelection,
        candles_provider: CandlesProviderBuffer,
    ) -> Self {
        Self {
            candles_selection,
            candles_provider,
        }
    }

    pub fn create_trader<T: TrendProvider + Send + Sync>(
        &self,
        trend_provider: T,
        trader_register: TraderRegister,
    ) -> Trader<T> {
        let mut candles_provider = self.candles_provider.clone();
        candles_provider.set_candles_selection(self.candles_selection);
        let indicator_provider = IndicatorProvider::new();

        Trader::new(
            trend_provider,
            self.candles_selection.symbol_minutes.symbol,
            indicator_provider,
            candles_provider,
            trader_register,
        )
    }
}
