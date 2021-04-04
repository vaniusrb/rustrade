use super::trend_provider::TrendProvider;
use crate::model::operation::Operation;
use crate::model::position::Position;
use crate::model::side::Side;
use crate::service::strategy::trade_context_provider::TradeContextProvider;
use crate::service::technicals::ind_type::IndicatorType;
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

        let _trend = if macd > macd_signal {
            Side::Bought
        } else {
            Side::Sold
        };

        debug!(
            "trend: {:?} {} > {}",
            trade_context_provider.now(),
            macd,
            macd_signal
        );
        Ok(None)
    }
}
