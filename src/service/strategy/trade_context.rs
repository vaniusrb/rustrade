use super::trend::trend_direction::TrendDirection;
use crate::service::candles_provider::{CandlesProvider, CandlesProviderSelection};
use crate::service::technicals::indicator::Indicator;
use crate::service::{
    candles_provider::CandlesProviderBuffer, technicals::ind_provider::IndicatorProvider,
};
use crate::{config::candles_selection::CandlesSelection, model::candle::Candle};
use crate::{model::price::Price, service::technicals::ind_type::IndicatorType};
use chrono::{DateTime, Utc};

pub struct TradeContext {
    symbol: i32,
    indicator_provider: IndicatorProvider,
    candles_provider: CandlesProviderBuffer,
    candles_opt: Option<(DateTime<Utc>, i32, Vec<Candle>)>,
    now: Option<DateTime<Utc>>,
    price: Option<Price>,
    current_trend_direction_opt: Option<TrendDirection>,
    trend_directions: Vec<TrendDirection>,
    changed_trend: Option<TrendDirection>,
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
            current_trend_direction_opt: None,
            trend_directions: Vec::new(),
            changed_trend: None,
        }
    }

    pub fn set_now(&mut self, now: DateTime<Utc>) {
        self.now = Some(now);
    }

    pub fn set_price(&mut self, price: Price) {
        self.price = Some(price);
    }

    fn normalized_trend(&self) -> Option<TrendDirection> {
        if self
            .trend_directions
            .iter()
            .all(|t| t == &TrendDirection::Sell)
        {
            Some(TrendDirection::Sell)
        } else if self
            .trend_directions
            .iter()
            .all(|t| t == &TrendDirection::Buy)
        {
            Some(TrendDirection::Buy)
        } else {
            None
        }
    }

    pub fn set_trend_direction(&mut self, trend_direction: TrendDirection) {
        if self.current_trend_direction_opt.is_none() {
            self.current_trend_direction_opt = Some(trend_direction.clone());
        }
        self.trend_directions.push(trend_direction);
        if self.trend_directions.len() > 3 {
            self.trend_directions.remove(0);
        }
        if let Some(normalized_trend) = self.normalized_trend() {
            let current_trend_direction = self.current_trend_direction_opt.as_ref().unwrap();
            if &normalized_trend != current_trend_direction {
                self.changed_trend = Some(normalized_trend.clone());
                self.current_trend_direction_opt = Some(normalized_trend);
            }
        }
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.now.unwrap()
    }

    pub fn price(&self) -> Price {
        self.price.unwrap()
    }

    pub fn changed_trend(&mut self) -> Option<TrendDirection> {
        self.changed_trend.take()
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
