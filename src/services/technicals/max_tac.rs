use super::{
    indicator::Indicator,
    serie::Serie,
    technical::{TechnicalDefinition, TechnicalIndicators},
};
use crate::config::definition::TacDefinition;
use crate::model::candle::Candle;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashMap;
use ta::{indicators::ExponentialMovingAverage as Max, Next};

pub const IND_MAX: &str = "max";

pub const TEC_MAX: &str = "max";

#[derive(Clone)]
pub struct MaxTac {
    pub indicators: HashMap<String, Indicator>,
}

impl TechnicalDefinition for MaxTac {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec![IND_MAX];
        TacDefinition::new(IND_MAX, &indicators)
    }
}

impl TechnicalIndicators for MaxTac {
    fn indicators(&self) -> &HashMap<String, Indicator> {
        &self.indicators
    }

    fn main_indicator(&self) -> &Indicator {
        self.indicators.get(IND_MAX).unwrap()
    }

    fn name(&self) -> String {
        TEC_MAX.to_string()
    }
}

impl<'a> MaxTac {
    pub fn new(candles: &[Candle], period: usize) -> Self {
        let mut max_series = Vec::with_capacity(candles.len());

        let mut indicators = HashMap::new();

        let mut max_ta = Max::new(period as usize).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();

            let max_result = max_ta.next(close);

            max_series.push(Serie::new(candle.close_time, max_result));
        }

        let max = Indicator::from(IND_MAX, max_series);
        indicators.insert(max.name.clone(), max);

        MaxTac { indicators }
    }

    pub fn _indicator(&self) -> &Indicator {
        self.indicators.get(IND_MAX).unwrap()
    }
}
