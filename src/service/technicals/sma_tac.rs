use super::{
    indicator::Indicator,
    serie::Serie,
    technical::{TechnicalDefinition, TechnicalIndicators},
};
use crate::config::definition::TacDefinition;
use crate::model::candle::Candle;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashMap;
use ta::{indicators::SimpleMovingAverage as Sma, Next};

pub const IND_SMA: &str = "sma";

pub const TAC_SMA: &str = "sma";

#[derive(Clone)]
pub struct SmaTac {
    pub indicators: HashMap<String, Indicator>,
}

impl TechnicalDefinition for SmaTac {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec![IND_SMA];
        TacDefinition::new(IND_SMA, &indicators)
    }
}

impl TechnicalIndicators for SmaTac {
    fn indicators(&self) -> &HashMap<String, Indicator> {
        &self.indicators
    }

    fn main_indicator(&self) -> &Indicator {
        self.indicators.get(IND_SMA).unwrap()
    }

    fn name(&self) -> String {
        todo!()
    }
}

impl<'a> SmaTac {
    pub fn new(candles: &[Candle], period: usize) -> Self {
        let mut sma_series = Vec::with_capacity(candles.len());

        let mut indicators = HashMap::new();

        let mut sma_ta = Sma::new(period as usize).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();
            let sma_result = sma_ta.next(close);

            sma_series.push(Serie::new(candle.close_time, sma_result));
        }

        let sma = Indicator::from(IND_SMA, sma_series);
        indicators.insert(sma.name.clone(), sma);

        Self { indicators }
    }
}
