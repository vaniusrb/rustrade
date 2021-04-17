use super::indicator::Indicator;
use super::technical::TecSerieIndicators;
use super::{
    serie::Serie,
    serie_indicator::SerieIndicator,
    technical::{TechnicalDefinition, TechnicalIndicators},
};
use crate::config::definition::TacDefinition;
use crate::model::candle::Candle;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashMap;
use ta::{indicators::RelativeStrengthIndex as Rsi, Next};

pub const IND_RSI: &str = "rsi";

pub const TEC_RSI: &str = "rsi";

pub struct RsiTac {
    pub serie_ind_map: HashMap<String, SerieIndicator>,
    //pub indicators_map: HashMap<String, &dyn Indicator>,
}

impl TechnicalDefinition for RsiTac {
    fn definition() -> TacDefinition {
        let indicators = vec![IND_RSI];
        TacDefinition::new(IND_RSI, &indicators)
    }
}

impl TechnicalIndicators for RsiTac {
    fn indicators(&self) -> &HashMap<String, SerieIndicator> {
        &self.serie_ind_map
    }

    fn main_indicator(&self) -> &dyn Indicator {
        let result = self.serie_ind_map.get(IND_RSI).unwrap();
        result as &dyn Indicator
    }

    fn name(&self) -> String {
        TEC_RSI.to_string()
    }
}

impl TecSerieIndicators for RsiTac {
    fn serie_indicators(&self) -> &HashMap<String, SerieIndicator> {
        &self.serie_ind_map
    }

    fn name(&self) -> String {
        TEC_RSI.to_string()
    }
}

impl<'a> RsiTac {
    pub fn get_indicator(&self, name: &str) -> Option<&dyn Indicator> {
        self.indicators().get(name).map(|s| s as &dyn Indicator)
    }

    pub fn new(candles: &[Candle], period: usize) -> Self {
        let mut rsi_series = Vec::with_capacity(candles.len());

        let mut indicators = HashMap::new();

        let mut rsi_ta = Rsi::new(period as usize).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();
            let rsi_result = rsi_ta.next(close);

            rsi_series.push(Serie::new(candle.close_time, rsi_result));
        }

        let name = IND_RSI.to_string();
        let rsi = SerieIndicator::from(&name, rsi_series);

        let rsi_ind = &rsi as &dyn Indicator;

        indicators.insert(name, rsi);

        // let mut indicators_map = HashMap::new();
        // indicators_map.insert(name, rsi_ind);

        Self {
            serie_ind_map: indicators,
            // indicators_map,
        }
    }
}
