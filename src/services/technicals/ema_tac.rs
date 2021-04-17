use super::indicator::Indicator;
use super::{
    serie::Serie,
    serie_indicator::SerieIndicator,
    technical::{TechnicalDefinition, TechnicalIndicators},
};
use crate::config::definition::TacDefinition;
use crate::model::candle::Candle;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashMap;
use ta::{indicators::ExponentialMovingAverage as Ema, Next};

pub const IND_EMA: &str = "ema";

pub const TEC_EMA: &str = "ema";

pub struct EmaTac {
    pub indicators: HashMap<String, SerieIndicator>,
}

impl TechnicalDefinition for EmaTac {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec![IND_EMA];
        TacDefinition::new(IND_EMA, &indicators)
    }
}

impl TechnicalIndicators for EmaTac {
    fn indicators(&self) -> &HashMap<String, SerieIndicator> {
        &self.indicators
    }

    fn main_indicator(&self) -> &dyn Indicator {
        self.indicators.get(IND_EMA).unwrap() as &dyn Indicator
    }

    fn name(&self) -> String {
        TEC_EMA.to_string()
    }
}

impl<'a> EmaTac {
    pub fn new(candles: &[Candle], period: usize) -> Self {
        let mut ema_series = Vec::with_capacity(candles.len());

        let mut indicators = HashMap::new();

        let mut ema_ta = Ema::new(period as usize).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();

            let ema_result = ema_ta.next(close);

            ema_series.push(Serie::new(candle.close_time, ema_result));
        }

        let ema = SerieIndicator::from(IND_EMA, ema_series);
        indicators.insert(IND_EMA.to_string(), ema);

        EmaTac { indicators }
    }

    pub fn main_serie_indicator(&self) -> &SerieIndicator {
        self.indicators.get(IND_EMA).unwrap()
    }
}
