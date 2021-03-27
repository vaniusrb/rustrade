use crate::application::candles_provider::CandlesProvider;
use crate::technicals::serie::Serie;
use crate::{
    config::definition::TacDefinition,
    technicals::{
        indicator::Indicator,
        technical::{TechnicalDefinition, TechnicalIndicators},
    },
};
use ifmt::iformat;
use log::debug;
use rust_decimal::prelude::ToPrimitive;
use std::{collections::HashMap, time::Instant};
use ta::{indicators::MovingAverageConvergenceDivergence as Macd, Next};

pub const MACD_IND: &str = "macd";
pub const MACD_SIG_IND: &str = "signal";
pub const MACD_DIV_IND: &str = "divergence";

#[derive(Clone)]
pub struct MacdTac {
    pub indicators: HashMap<String, Indicator>,
}

impl TechnicalDefinition for MacdTac {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec![MACD_IND, MACD_SIG_IND, MACD_DIV_IND];
        TacDefinition::new(MACD_IND, &indicators)
    }
}
impl TechnicalIndicators for MacdTac {
    fn main_indicator(&self) -> &Indicator {
        self.indicators.get(MACD_IND).unwrap()
    }

    fn indicators(&self) -> &HashMap<String, Indicator> {
        &self.indicators
    }
}

impl<'a> MacdTac {
    pub fn new(
        mut candles_provider: Box<dyn CandlesProvider>,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> Self {
        let start = Instant::now();

        let candles = candles_provider.candles().unwrap();

        let mut macd_series = Vec::with_capacity(candles.len());
        let mut signal_series = Vec::with_capacity(candles.len());
        let mut divergence_series = Vec::with_capacity(candles.len());

        let mut indicators = HashMap::new();

        // 34, 72, 17
        let mut macd_ta = Macd::new(fast_period, slow_period, signal_period).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();

            let macd_result: (f64, f64, f64) = macd_ta.next(close).into();

            macd_series.push(Serie::new(candle.close_time, macd_result.0));
            signal_series.push(Serie::new(candle.close_time, macd_result.1));
            divergence_series.push(Serie::new(candle.close_time, macd_result.2));
        }

        let macd = Indicator::from(MACD_IND, macd_series);
        let signal = Indicator::from(MACD_SIG_IND, signal_series);
        let divergence = Indicator::from(MACD_DIV_IND, divergence_series);

        indicators.insert(macd.name.clone(), macd);
        indicators.insert(signal.name.clone(), signal);
        indicators.insert(divergence.name.clone(), divergence);

        debug!("{}", iformat!("macd load {candles.len()}: {start.elapsed():?}"));

        MacdTac { indicators }
    }
}
