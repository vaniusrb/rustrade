use super::{trade_context::TradeContext, trend::trend_direction::TrendDirection};
use crate::services::provider::candles_provider::CandlesProviderBuffer;
use crate::services::technicals::ind_type::IndicatorType;
use crate::services::technicals::indicator::Indicator;
use crate::{model::price::Price, services::technicals::ind_provider::IndicatorProvider};
use chrono::{DateTime, Utc};
use std::rc::Rc;
use std::{cell::Cell, sync::Mutex};

pub struct TradeContextProvider {
    trade_context: Rc<Mutex<Cell<TradeContext>>>,
}

impl Clone for TradeContextProvider {
    fn clone(&self) -> Self {
        Self {
            trade_context: self.trade_context.clone(),
        }
    }
}

impl TradeContextProvider {
    pub fn new(
        symbol: i32,
        indicator_provider: IndicatorProvider,
        candles_provider: CandlesProviderBuffer,
    ) -> Self {
        Self {
            trade_context: Rc::new(Mutex::new(Cell::new(TradeContext::new(
                symbol,
                indicator_provider,
                candles_provider,
            )))),
        }
    }

    pub fn set_now(&self, now: DateTime<Utc>) {
        self.trade_context.lock().unwrap().get_mut().set_now(now);
    }

    pub fn changed_trend(&self) -> Option<TrendDirection> {
        self.trade_context.lock().unwrap().get_mut().changed_trend()
    }

    pub fn set_trend_direction(&self, trend_direction: TrendDirection) {
        self.trade_context
            .lock()
            .unwrap()
            .get_mut()
            .set_trend_direction(trend_direction);
    }

    pub fn set_price(&self, price: Price) {
        self.trade_context
            .lock()
            .unwrap()
            .get_mut()
            .set_price(price);
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.trade_context.lock().unwrap().get_mut().now()
    }

    pub fn price(&self) -> Price {
        self.trade_context.lock().unwrap().get_mut().price()
    }

    pub fn value(&self, minutes: i32, i_type: &IndicatorType) -> eyre::Result<f64> {
        self.trade_context
            .lock()
            .unwrap()
            .get_mut()
            .indicator(minutes, i_type)
            .unwrap()
            .value()
    }
}
