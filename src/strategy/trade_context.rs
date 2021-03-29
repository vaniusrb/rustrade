use crate::{
    application::candles_provider::{CandlesProvider, CandlesProviderBuffer, CandlesProviderSelection},
    config::candles_selection::CandlesSelection,
    model::candle::Candle,
    technicals::{ind_provider::IndicatorProvider, ind_type::IndicatorType, indicator::Indicator},
};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct TradeContext {
    symbol: String,
    indicator_provider: IndicatorProvider,
    candles_provider: CandlesProviderBuffer,
    candles_opt: Option<(DateTime<Utc>, u32, Vec<Candle>)>,
    now: Option<DateTime<Utc>>,
}

impl TradeContext {
    pub fn new(symbol: &str, indicator_provider: IndicatorProvider, candles_provider: CandlesProviderBuffer) -> Self {
        Self {
            symbol: symbol.to_string(),
            indicator_provider,
            candles_provider,
            candles_opt: None,
            now: None,
        }
    }

    pub fn set_now(&mut self, now: DateTime<Utc>) {
        self.now = Some(now);
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.now.unwrap()
    }

    pub fn indicator(&mut self, minutes: u32, i_type: &IndicatorType) -> anyhow::Result<&Indicator> {
        let now = self.now();
        self.candles_opt = self.candles_opt.take().filter(|e| e.0 == now && e.1 == minutes);

        let candles_provider = &self.candles_provider;
        let symbol = &self.symbol;

        let now_candles = self.candles_opt.get_or_insert_with(|| {
            let candles_selection = CandlesSelection::last_n(symbol, &minutes, 200, now);
            let mut candles_provider_selection =
                CandlesProviderSelection::new(candles_provider.clone(), candles_selection);
            let candles = candles_provider_selection.candles().unwrap();
            (now, minutes, candles)
        });
        self.indicator_provider.indicator(now, &now_candles.2, i_type)
    }
}
