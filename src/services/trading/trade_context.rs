use super::trend::trend_direction::TrendDirection;
use crate::services::provider::candles_provider::CandlesProvider;
use crate::services::provider::candles_provider_buffer::CandlesProviderBuffer;
use crate::services::technicals::ind_provider::IndicatorProvider;
use crate::services::technicals::indicator::Indicator;
use crate::{config::candles_selection::CandlesSelection, model::candle::Candle};
use crate::{model::price::Price, services::technicals::ind_type::IndicatorType};
use chrono::{DateTime, Utc};

pub struct TradeContext {
    symbol: i32,
    indicator_provider: IndicatorProvider,
    candles_provider: CandlesProviderBuffer,
    candles_opt: Option<(Vec<Candle>, DateTime<Utc>, i32, i32)>,
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
            self.current_trend_direction_opt = Some(trend_direction);
        }
        self.trend_directions.push(trend_direction);
        if self.trend_directions.len() > 3 {
            self.trend_directions.remove(0);
        }
        if let Some(normalized_trend) = self.normalized_trend() {
            let current_trend_direction = self.current_trend_direction_opt.as_ref().unwrap();
            if &normalized_trend != current_trend_direction {
                self.changed_trend = Some(normalized_trend);
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

    pub fn indicator(
        &mut self,
        minutes: i32,
        indicator_type: &IndicatorType,
    ) -> eyre::Result<&dyn Indicator> {
        let now = self.now();
        let period = indicator_type.period();
        // This caching is working ok
        self.candles_opt = self
            .candles_opt
            .take()
            .filter(|e| e.1 == now && e.2 == minutes && e.3 == period);

        let candles_provider = &mut self.candles_provider;
        let symbol = self.symbol;

        let (candles, _, _, _) = self.candles_opt.get_or_insert_with(|| {
            let candles_selection = CandlesSelection::last_n(symbol, minutes, period, now);
            // TODO here should considere use range
            // let mut candles_provider_selection =
            //     CandlesProviderSelection::new(candles_provider.clone(), candles_selection);
            // let candles = candles_provider_selection.candles().unwrap();

            candles_provider.set_candles_selection(candles_selection);
            let candles = candles_provider.candles().unwrap();

            (candles, now, minutes, period)
        });
        self.indicator_provider
            .indicator(now, candles, indicator_type)
    }
}
