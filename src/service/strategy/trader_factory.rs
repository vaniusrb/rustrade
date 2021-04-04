use super::{
    trade_operation::Trader, trader_register::TraderRegister, trend::trend_provider::TrendProvider,
};
use crate::config::candles_selection::CandlesSelection;
use crate::service::candles_provider::CandlesProviderBuffer;
use crate::service::technicals::ind_provider::IndicatorProvider;

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

    pub fn create_trader(
        &self,
        trend_provider: Box<dyn TrendProvider + Send + Sync>,
        trader_register: TraderRegister,
    ) -> Trader {
        let mut candles_provider = self.candles_provider.clone();
        candles_provider.set_candles_selection(self.candles_selection.clone());
        let indicator_provider = IndicatorProvider::new();

        Trader::new(
            trend_provider,
            &self.candles_selection.symbol_minutes.symbol,
            indicator_provider,
            candles_provider,
            trader_register,
        )
    }
}
