use super::{trader_register::Trade, trend::trend_provider::TrendProvider, trend_enum::Trend};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct Trader {
    trend_provider: Box<dyn TrendProvider + Send + Sync>,
    previous_trend: Option<Trend>,
    trades: Vec<Trade>,
}

impl<'a> Trader {
    pub fn new(trend_provider: Box<dyn TrendProvider + Send + Sync>) -> Self {
        Self {
            trend_provider,
            previous_trend: None,
            trades: Vec::new(),
        }
    }

    pub fn check(&'a mut self, now: DateTime<Utc>, price: Decimal) -> anyhow::Result<()> {
        let trend_provider = &mut self.trend_provider;

        let trade_context_provider = trend_provider.trade_context_provider();
        trade_context_provider.set_now(now);

        let trend = trend_provider.trend()?;

        let previous_trend = self.previous_trend.get_or_insert_with(|| trend.clone());

        if &trend != previous_trend {
            let trade = Trade::new(trend.to_operation(), now, price);
            self.trades.push(trade);
        }
        self.previous_trend = Some(trend);
        Ok(())
    }

    pub fn trades(&self) -> Vec<Trade> {
        self.trades.clone()
    }
}
