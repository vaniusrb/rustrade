use super::{
    trade_operation::TradeOperation, trader_register::TraderRegister,
    trend::trend_provider::TrendProvider,
};
use crate::services::provider::candles_provider_buffer::CandlesProviderBuffer;
use crate::services::trading::trade_context_provider::TradeContextProvider;
use crate::{model::price::Price, services::technicals::ind_provider::IndicatorProvider};
use chrono::{DateTime, Utc};

pub struct Trader<T: TrendProvider + Send + Sync> {
    trend_provider: T,
    trade_operations: Vec<TradeOperation>,
    trade_context_provider: TradeContextProvider,
    trader_register: TraderRegister,
}

impl<'a, T: TrendProvider + Send + Sync> Trader<T> {
    pub fn new(
        trend_provider: T,
        symbol: i32,
        indicator_provider: IndicatorProvider,
        candles_provider: CandlesProviderBuffer,
        trader_register: TraderRegister,
    ) -> Self {
        let trade_context_provider =
            TradeContextProvider::new(symbol, indicator_provider, candles_provider);
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

        let position = self.trader_register.position_register();

        let running_script_state = self
            .trend_provider
            .trend(position, &self.trade_context_provider)?;

        self.trade_context_provider
            .set_trend_direction(running_script_state.trend_direction);

        // Get the operation, if none then exit routine
        let trade_operation = match running_script_state.trade_operation_opt {
            Some(trade_operation) => trade_operation,
            None => return Ok(()),
        };

        self.trader_register.register(trade_operation.clone())?;

        self.trade_operations.push(trade_operation);
        Ok(())
    }

    pub fn trades(&self) -> Vec<TradeOperation> {
        self.trade_operations.clone()
    }
}
