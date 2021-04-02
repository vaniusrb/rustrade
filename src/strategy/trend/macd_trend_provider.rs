use super::trend_provider::TrendProvider;
use crate::{
    strategy::{
        trade_context_provider::TradeContextProvider,
        trend_enum::{Operation, Side},
    },
    technicals::ind_type::IndicatorType,
};
use log::debug;

/// setup
/// transfer 1000 USD
/// buy 500 USD

#[derive(Clone)]
pub struct MacdTrendProvider {}

impl MacdTrendProvider {
    pub fn from() -> Self {
        Self {}
    }
}

impl TrendProvider for MacdTrendProvider {
    fn trend(&mut self, trade_context_provider: &TradeContextProvider) -> eyre::Result<Option<Operation>> {
        let mcad = trade_context_provider
            .indicator(15, &IndicatorType::Macd(34, 72, 17))?
            .value()?;

        let mcad_signal = trade_context_provider
            .indicator(15, &IndicatorType::MacdSignal(34, 72, 17))?
            .value()?;

        //let _mcad_divergence = trend_context_provider.indicator(15, &IndicatorType::MacdDivergence(34, 72, 17))?.value()?;
        let trend = if mcad > mcad_signal { Side::Bought } else { Side::Sold };

        // TODO
        debug!("trend: {:?} {} > {}", trade_context_provider.now(), mcad, mcad_signal);
        Ok(None)
    }
}
