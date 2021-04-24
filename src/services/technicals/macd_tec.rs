use crate::services::technicals::serie::Serie;
use crate::{
    config::definition::TacDefinition,
    services::technicals::{serie_indicator::SerieIndicator, technical::TechnicalIndicators},
};
use crate::{model::candle::Candle, services::technicals::technical::TechnicalDefinition};
use ifmt::iformat;
use log::debug;
use rust_decimal::prelude::ToPrimitive;
use std::{collections::HashMap, time::Instant};
use ta::{indicators::MovingAverageConvergenceDivergence as Macd, Next};

use super::indicator::Indicator;
use super::technical::TecSerieIndicators;

pub const IND_MACD: &str = "macd";
pub const IND_MACD_SIG: &str = "signal";
pub const IND_MACD_DIV: &str = "divergence";

pub const TEC_MCAD: &str = "macd";

pub struct MacdTec {
    indicators: HashMap<String, SerieIndicator>,
}

impl TechnicalDefinition for MacdTec {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec![IND_MACD, IND_MACD_SIG, IND_MACD_DIV];
        TacDefinition::new(IND_MACD, &indicators)
    }
}

impl TechnicalIndicators for MacdTec {
    fn get_indicator(&self, name: &str) -> Option<&dyn Indicator> {
        self.indicators.get(name).map(|s| s as &dyn Indicator)
    }

    fn main_indicator(&self) -> &dyn Indicator {
        let result = self.indicators.get(IND_MACD).unwrap();
        result as &(dyn Indicator)
    }

    fn name(&self) -> String {
        TEC_MCAD.to_string()
    }
}

impl TecSerieIndicators for MacdTec {
    fn serie_indicators(&self) -> &HashMap<String, SerieIndicator> {
        &self.indicators
    }

    fn name(&self) -> String {
        TEC_MCAD.to_string()
    }
}

impl<'a> MacdTec {
    pub fn new(
        candles: &[Candle],
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> Self {
        let start = Instant::now();

        let mut macd_series = Vec::with_capacity(candles.len());
        let mut signal_series = Vec::with_capacity(candles.len());
        let mut divergence_series = Vec::with_capacity(candles.len());

        let mut indicators = HashMap::new();

        // Default values are 34, 72, 17
        let mut macd_ta = Macd::new(fast_period, slow_period, signal_period).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();

            let macd_result: (f64, f64, f64) = macd_ta.next(close).into();

            macd_series.push(Serie::new(candle.close_time, macd_result.0));
            signal_series.push(Serie::new(candle.close_time, macd_result.1));
            divergence_series.push(Serie::new(candle.close_time, macd_result.2));
        }

        let macd = SerieIndicator::from(IND_MACD, macd_series);
        let signal = SerieIndicator::from(IND_MACD_SIG, signal_series);
        let divergence = SerieIndicator::from(IND_MACD_DIV, divergence_series);

        indicators.insert(IND_MACD.to_string(), macd);
        indicators.insert(IND_MACD_SIG.to_string(), signal);
        indicators.insert(IND_MACD_DIV.to_string(), divergence);

        debug!(
            "{}",
            iformat!("macd load {candles.len()}: {start.elapsed():?}")
        );

        MacdTec { indicators }
    }
}
