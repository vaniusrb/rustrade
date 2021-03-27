use std::{cell::RefCell, sync::Arc};

use crate::{
    application::candles_provider::CandlesProviderBuffer,
    technicals::{ind_provider::IndicatorProvider, ind_type::IndicatorType, indicator::Indicator},
};
use chrono::{DateTime, Utc};

use super::trade_context::TradeContext;

#[derive(Clone)]
pub struct TradeContextProvider {
    trade_context: Arc<RefCell<TradeContext>>,
}

impl TradeContextProvider {
    pub fn new(symbol: &str, indicator_provider: IndicatorProvider, candles_provider: CandlesProviderBuffer) -> Self {
        Self {
            trade_context: Arc::new(RefCell::new(TradeContext::new(
                symbol,
                indicator_provider,
                candles_provider,
            ))),
        }
    }

    pub fn set_now(&self, now: DateTime<Utc>) {
        self.trade_context.borrow_mut().set_now(now);
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.trade_context.borrow_mut().now()
    }

    pub fn indicator(&self, minutes: u32, i_type: &IndicatorType) -> anyhow::Result<&Indicator> {
        self.trade_context.borrow_mut().indicator(minutes, i_type)
    }
}
