use crate::service::candles_provider::{CandlesProvider, CandlesProviderSelection};
use crate::service::technicals::indicator::Indicator;
use crate::service::{
    candles_provider::CandlesProviderBuffer, technicals::ind_provider::IndicatorProvider,
};
use crate::{config::candles_selection::CandlesSelection, model::candle::Candle};
use crate::{model::price::Price, service::technicals::ind_type::IndicatorType};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct TradeContext {
    symbol: i32,
    indicator_provider: IndicatorProvider,
    candles_provider: CandlesProviderBuffer,
    candles_opt: Option<(DateTime<Utc>, i32, Vec<Candle>)>,
    now: Option<DateTime<Utc>>,
    price: Option<Price>,
}

impl TradeContext {
    pub fn new(
        symbol: i32,
        indicator_provider: IndicatorProvider,
        candles_provider: CandlesProviderBuffer,
    ) -> Self {
        Self {
            symbol,
            indicator_provider,
            candles_provider,
            candles_opt: None,
            now: None,
            price: None,
        }
    }

    pub fn set_now(&mut self, now: DateTime<Utc>) {
        self.now = Some(now);
    }

    pub fn set_price(&mut self, price: Price) {
        self.price = Some(price);
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.now.unwrap()
    }

    pub fn price(&self) -> Price {
        self.price.unwrap()
    }

    // TODO Should use max period from indicator
    const LAST_CANDLES_INDICATOR: i32 = 100;

    pub fn indicator(&mut self, minutes: i32, i_type: &IndicatorType) -> eyre::Result<&Indicator> {
        let now = self.now();
        self.candles_opt = self
            .candles_opt
            .take()
            .filter(|e| e.0 == now && e.1 == minutes);

        let candles_provider = &self.candles_provider;
        let symbol = &self.symbol;

        let now_candles = self.candles_opt.get_or_insert_with(|| {
            let candles_selection =
                CandlesSelection::last_n(*symbol, minutes, Self::LAST_CANDLES_INDICATOR, now);
            let mut candles_provider_selection =
                CandlesProviderSelection::new(candles_provider.clone(), candles_selection);
            let candles = candles_provider_selection.candles().unwrap();
            (now, minutes, candles)
        });
        self.indicator_provider
            .indicator(now, &now_candles.2, i_type)
    }
}
