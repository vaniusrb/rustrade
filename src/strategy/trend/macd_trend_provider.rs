use super::trend_provider::TrendProvider;
use crate::strategy::operation::Operation;
use crate::{
    strategy::{position::Position, side::Side, trade_context_provider::TradeContextProvider},
    technicals::ind_type::IndicatorType,
};
use log::debug;

#[derive(Clone)]
pub struct MacdTrendProvider {}

impl MacdTrendProvider {
    pub fn from() -> Self {
        Self {}
    }
}

impl TrendProvider for MacdTrendProvider {
    fn trend(
        &mut self,
        _position: &Position,
        trade_context_provider: &TradeContextProvider,
    ) -> eyre::Result<Option<Operation>> {
        let macd = trade_context_provider
            .indicator(15, &IndicatorType::Macd(34, 72, 17))?
            .value()?;

        let macd_signal = trade_context_provider
            .indicator(15, &IndicatorType::MacdSignal(34, 72, 17))?
            .value()?;

        let _trend = if macd > macd_signal { Side::Bought } else { Side::Sold };

        debug!("trend: {:?} {} > {}", trade_context_provider.now(), macd, macd_signal);
        Ok(None)
    }
}
