use super::{
    trade_operation::TradeOperation, trader_register::TraderRegister,
    trend::trend_provider::TrendProvider,
};
use crate::services::provider::candles_provider::CandlesProviderBuffer;
use crate::services::trader::trade_context_provider::TradeContextProvider;
use crate::{model::price::Price, services::technicals::ind_provider::IndicatorProvider};
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

        let operation = match running_script_state.operation_opt {
            Some(operation) => operation,
            None => return Ok(()),
        };

        let trade_operation = TradeOperation::new(operation, now, price);

        self.trader_register.register(trade_operation)?;

        self.trade_operations.push(trade_operation);
        Ok(())
    }

    pub fn trades(&self) -> Vec<TradeOperation> {
        self.trade_operations.clone()
    }
}
