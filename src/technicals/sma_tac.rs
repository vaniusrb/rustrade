use crate::model::candle::Candle;
use crate::{config::definition::TacDefinition, technicals::indicator::Indicator};
use ifmt::iformat;
use log::info;
use rust_decimal::prelude::ToPrimitive;
use std::{collections::HashMap, time::Instant};
use ta::{indicators::SimpleMovingAverage as Sma, Next};

use super::technical::{TechnicalDefinition, TechnicalIndicators};

pub const SMA_IND: &str = "sma";
pub struct SmaTac {
    pub indicators: HashMap<String, Indicator>,
}

impl TechnicalDefinition for SmaTac {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec![SMA_IND];
        TacDefinition::new(SMA_IND, &indicators)
    }
}

impl TechnicalIndicators for SmaTac {
    fn indicators(&self) -> &HashMap<String, Indicator> {
        &self.indicators
    }

    fn main_indicator(&self) -> &Indicator {
        self.indicators.get(SMA_IND).unwrap()
    }
}

impl<'a> SmaTac {
    // default period is 34
    pub fn new(candles: &'a [&'a Candle], period: usize) -> Self {
        let start = Instant::now();

        let mut sma = Indicator::new(SMA_IND, candles.len());
        let mut indicators = HashMap::new();

        let mut sma_ta = Sma::new(period as usize).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();
            let sma_result = sma_ta.next(close);
            sma.push_serie(candle.close_time, sma_result);
        }

        indicators.insert(sma.name.clone(), sma);

        info!("{}", iformat!("Technicals {candles.len()}: {start.elapsed():?}"));

        Self { indicators }
    }
}
