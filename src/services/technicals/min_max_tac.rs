use super::{
    indicator::Indicator,
    serie::Serie,
    technical::{TechnicalDefinition, TechnicalIndicators},
};
use crate::config::definition::TacDefinition;
use crate::model::candle::Candle;
use crate::services::technicals::value_indicator::ValueIndicator;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;
use std::collections::HashMap;

pub const IND_MIN_MAX: &str = "min_max";

pub const TEC_MIN_MAX: &str = "min_max";

pub struct MinMaxTac {
    pub indicators: HashMap<String, ValueIndicator>,
}

impl TechnicalDefinition for MinMaxTac {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec![IND_MIN_MAX];
        TacDefinition::new(IND_MIN_MAX, &indicators)
    }
}

// impl TechnicalIndicators for MinMaxTac {
//     fn indicators(&self) -> &HashMap<String, SerieIndicator> {
//         &self.indicators
//     }

//     fn main_indicator(&self) -> &dyn Indicator {
//         &**self.indicators.get(IND_MIN_MAX).unwrap()
//     }

//     fn name(&self) -> String {
//         TEC_MIN_MAX.to_string()
//     }
// }

impl<'a> MinMaxTac {
    pub fn new(candles: &[Candle], period: usize) -> Self {
        let start = (candles.len() - period).max(0);

        let last_candles = candles[start..candles.len()].to_vec();

        let max = last_candles.iter().fold(dec!(0), |acc, x| acc.max(x.high));
        let min = last_candles.iter().fold(max, |acc, x| acc.min(x.low));

        let mut indicators = HashMap::new();

        let ind_max = ValueIndicator::new(max.to_f64().unwrap());
        indicators.insert("max".to_string(), ind_max);

        let ind_min = ValueIndicator::new(min.to_f64().unwrap());
        indicators.insert("min".to_string(), ind_min);

        Self { indicators }
    }
}
