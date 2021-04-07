use super::{
    indicator::Indicator,
    serie::Serie,
    technical::{TechnicalDefinition, TechnicalIndicators},
};
use crate::config::definition::TacDefinition;
use crate::model::candle::Candle;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashMap;
use ta::{indicators::RelativeStrengthIndex as Rsi, Next};

pub const RSI_IND: &str = "rsi";

#[derive(Clone)]
pub struct RsiTac {
    pub indicators: HashMap<String, Indicator>,
}

impl TechnicalDefinition for RsiTac {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec![RSI_IND];
        TacDefinition::new(RSI_IND, &indicators)
    }
}

impl TechnicalIndicators for RsiTac {
    fn indicators(&self) -> &HashMap<String, Indicator> {
        &self.indicators
    }

    fn main_indicator(&self) -> &Indicator {
        self.indicators.get(RSI_IND).unwrap()
    }
}

impl<'a> RsiTac {
    pub fn new(candles: &[Candle], period: usize) -> Self {
        let mut rsi_series = Vec::with_capacity(candles.len());

        let mut indicators = HashMap::new();

        let mut rsi_ta = Rsi::new(period as usize).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();
            let rsi_result = rsi_ta.next(close);

            rsi_series.push(Serie::new(candle.close_time, rsi_result));
        }

        let rsi = Indicator::from(RSI_IND, rsi_series);
        indicators.insert(rsi.name.clone(), rsi);

        Self { indicators }
    }
}
