use crate::model::candle::Candle;
use crate::{config::definition::TacDefinition, technicals::indicator::Indicator};
use ifmt::iformat;
use log::info;
use rust_decimal::prelude::ToPrimitive;
use std::{collections::HashMap, time::Instant};
use ta::{indicators::SimpleMovingAverage as Sma, Next};

use super::technical::{TechnicalDefinition, TechnicalIndicators};

pub struct SmaTac<'a> {
    pub indicators: HashMap<String, Indicator<'a>>,
}

impl<'a> TechnicalDefinition<'a> for SmaTac<'a> {
    fn definition() -> crate::config::definition::TacDefinition {
        let indicators = vec!["sma"];
        TacDefinition::new("sma", &indicators)
    }
}

impl<'a> TechnicalIndicators<'a> for SmaTac<'a> {
    fn indicators(&self) -> &HashMap<String, Indicator<'a>> {
        &self.indicators
    }

    fn main_indicator(&self) -> &Indicator {
        self.indicators.get("sma").unwrap()
    }
}

impl<'a> SmaTac<'a> {
    // default period is 34
    pub fn new(candles: &'a [&'a Candle], period: usize) -> Self {
        let start = Instant::now();

        let mut sma = Indicator::new("sma");
        let mut indicators = HashMap::new();

        let mut sma_ta = Sma::new(period as usize).unwrap();
        for candle in candles.iter() {
            let close = candle.close.to_f64().unwrap();

            let sma_result = sma_ta.next(close);
            sma.push_serie(&candle.close_time, sma_result);
        }

        indicators.insert(sma.name.clone(), sma);

        info!("{}", iformat!("Technicals {candles.len()}: {start.elapsed():?}"));

        Self { indicators }
    }
}

/*
    java:
    /home/vanius/Documents/java/TradeBot/src/main/java/br/com/vanius/tradebot/trader/TraderMACD.java
        MACDIndicator macd = new MACDIndicator(closePriceIndicator, 12, 26);
        EMAIndicator sma = new EMAIndicator(macd, 9);
        currentInd = currentMACD.subtract(currentSMA);

    rust:

                        // let ema_val = ema_9.next(&dt);

                        // 17,34,72
                        // let mut macd = Macd::new(3, 6, 4).unwrap();
                        // macd.next(&dt);

*/