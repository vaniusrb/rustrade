use super::{
    trade_context_provider::TradeContextProvider,
    trader_register::{TradeOperation, TraderRegister},
    trend::trend_provider::TrendProvider,
    trend_enum::Trend,
};
use crate::model::quantity::Quantity;
use crate::{
    application::candles_provider::CandlesProviderBuffer, model::price::Price,
    technicals::ind_provider::IndicatorProvider,
};
use chrono::{DateTime, Utc};
use rust_decimal_macros::dec;

pub struct Trader {
    trend_provider: Box<dyn TrendProvider + Send + Sync>,
    previous_trend: Option<Trend>,
    trade_operations: Vec<TradeOperation>,
    trade_context_provider: TradeContextProvider,
    trader_register: TraderRegister,
}

impl<'a> Trader {
    pub fn new(
        trend_provider: Box<dyn TrendProvider + Send + Sync>,
        symbol: &str,
        indicator_provider: IndicatorProvider,
        candles_provider: CandlesProviderBuffer,
        trader_register: TraderRegister,
    ) -> Self {
        let trade_context_provider = TradeContextProvider::new(symbol, indicator_provider, candles_provider);

        Self {
            trend_provider,
            previous_trend: None,
            trade_operations: Vec::new(),
            trade_context_provider,
            trader_register,
        }
    }

    pub fn check(&'a mut self, now: DateTime<Utc>, price: Price) -> eyre::Result<()> {
        self.trade_context_provider.set_now(now);
        let trend = self.trend_provider.trend(&self.trade_context_provider)?;

        // TODO Trend should by like Order
        let quantity = Quantity(dec!(1));
        let previous_trend = self.previous_trend.get_or_insert_with(|| trend.clone());

        if &trend != previous_trend {
            let trade_operation = TradeOperation::new(trend.to_operation(), now, quantity, price);

            self.trader_register.register(trade_operation.clone());

            self.trade_operations.push(trade_operation);
        }
        self.previous_trend = Some(trend);
        Ok(())
    }

    pub fn trades(&self) -> Vec<TradeOperation> {
        self.trade_operations.clone()
    }
}
