use super::{
    trade_context_provider::TradeContextProvider,
    trader_register::{TradeOperation, TraderRegister},
    trend::trend_provider::TrendProvider,
};
use crate::{
    application::candles_provider::CandlesProviderBuffer, model::price::Price,
    technicals::ind_provider::IndicatorProvider,
};
use chrono::{DateTime, Utc};

pub struct Trader {
    trend_provider: Box<dyn TrendProvider + Send + Sync>,
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
            trade_operations: Vec::new(),
            trade_context_provider,
            trader_register,
        }
    }

    pub fn check(&'a mut self, now: DateTime<Utc>, price: Price) -> eyre::Result<()> {
        self.trade_context_provider.set_now(now);
        self.trade_context_provider.set_price(price);

        let position = self.trader_register.position();

        let operation_opt = self.trend_provider.trend(position, &self.trade_context_provider)?;
        let operation = match operation_opt {
            Some(operation) => operation,
            None => return Ok(()),
        };

        let trade_operation = TradeOperation::new(operation, now, price);

        self.trader_register.register(trade_operation);

        self.trade_operations.push(trade_operation);
        Ok(())
    }

    pub fn trades(&self) -> Vec<TradeOperation> {
        self.trade_operations.clone()
    }
}
