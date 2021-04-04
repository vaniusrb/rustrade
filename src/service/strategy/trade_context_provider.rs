use super::trade_context::TradeContext;
use crate::service::candles_provider::CandlesProviderBuffer;
use crate::service::technicals::ind_type::IndicatorType;
use crate::service::technicals::indicator::Indicator;
use crate::{model::price::Price, service::technicals::ind_provider::IndicatorProvider};
use chrono::{DateTime, Utc};
use std::{cell::Cell, sync::Mutex};

pub struct TradeContextProvider {
    trade_context: Mutex<Cell<TradeContext>>,
}

impl Clone for TradeContextProvider {
    fn clone(&self) -> Self {
        Self {
            trade_context: Mutex::new(Cell::new(
                self.trade_context.lock().unwrap().get_mut().clone(),
            )),
        }
    }
}

impl TradeContextProvider {
    pub fn new(
        symbol: &str,
        indicator_provider: IndicatorProvider,
        candles_provider: CandlesProviderBuffer,
    ) -> Self {
        Self {
            trade_context: Mutex::new(Cell::new(TradeContext::new(
                symbol,
                indicator_provider,
                candles_provider,
            ))),
        }
    }

    pub fn set_now(&self, now: DateTime<Utc>) {
        self.trade_context.lock().unwrap().get_mut().set_now(now);
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

    pub fn indicator(&self, minutes: u32, i_type: &IndicatorType) -> eyre::Result<Indicator> {
        self.trade_context
            .lock()
            .unwrap()
            .get_mut()
            .indicator(minutes, i_type)
            .map(|i| i.clone())
    }
}
