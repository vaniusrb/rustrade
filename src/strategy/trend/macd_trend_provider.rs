use super::trend_provider::TrendProvider;
use crate::{
    strategy::{trade_context_provider::TradeContextProvider, trend_enum::Trend},
    technicals::ind_type::IndicatorType,
};
use log::debug;

/// setup
/// transfer 1000 USD
/// buy 500 USD

#[derive(Clone)]
pub struct MacdTrendProvider {
    trade_context_provider: TradeContextProvider,
}

impl MacdTrendProvider {
    pub fn from(trend_context_provider: TradeContextProvider) -> Self {
        Self {
            trade_context_provider: trend_context_provider,
        }
    }
}

impl TrendProvider for MacdTrendProvider {
    fn trend(&mut self) -> anyhow::Result<Trend> {
        let mcad = self.trade_context_provider.indicator(15, &IndicatorType::Macd(34, 72, 17))?.value()?;
        let mcad_signal = self.trade_context_provider.indicator(15, &IndicatorType::MacdSignal(34, 72, 17))?.value()?;
        //let _mcad_divergence = trend_context_provider.indicator(15, &IndicatorType::MacdDivergence(34, 72, 17))?.value()?;
        let trend = if mcad > mcad_signal { Trend::Bought } else { Trend::Sold };

        debug!("trend: {:?} {} > {}", self.trade_context_provider.now(), mcad, mcad_signal);
        Ok(trend)
    }

    fn trade_context_provider(&mut self) -> &mut TradeContextProvider {
        &mut self.trade_context_provider
    }
}
