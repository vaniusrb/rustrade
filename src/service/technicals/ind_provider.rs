use super::{
    ema_tac::{EmaTac, EMA_IND},
    ind_type::IndicatorType,
    macd::macd_tac::{MacdTac, MACD_DIV_IND, MACD_IND, MACD_SIG_IND},
    sma_tac::{SmaTac, SMA_IND},
    technical::TechnicalIndicators,
};
use crate::model::candle::Candle;
use crate::service::technicals::indicator::Indicator;
use chrono::{DateTime, Utc};
use eyre::eyre;
use std::collections::HashMap;

pub struct IndicatorProvider {
    macds_opt: Option<(DateTime<Utc>, usize, usize, usize, MacdTac)>,
    tac_indicators:
        HashMap<(String, usize), eyre::Result<Box<dyn TechnicalIndicators + Send + Sync>>>, // <= to allow trait with different lifetime
}

impl Clone for IndicatorProvider {
    fn clone(&self) -> Self {
        Self {
            macds_opt: self.macds_opt.clone(),
            tac_indicators: HashMap::new(),
        }
    }
}

impl IndicatorProvider {
    pub fn new() -> Self {
        Self {
            macds_opt: None,
            tac_indicators: HashMap::new(),
        }
    }
    fn tac_indicator(
        &mut self,
        candles: &[Candle],
        ind_name: &str,
        period: usize,
    ) -> eyre::Result<&Indicator> {
        self.tac_indicators.clear();
        // TODO I shouldn't store Indicator cache, or use "now" like a key
        let result: &mut eyre::Result<Box<dyn TechnicalIndicators + Send + Sync>> = self
            .tac_indicators
            .entry((ind_name.to_string(), period))
            .or_insert_with(|| {
                let result: eyre::Result<Box<dyn TechnicalIndicators + Send + Sync>> =
                    match ind_name {
                        EMA_IND => Ok(Box::new(EmaTac::new(candles, period))
                            as Box<dyn TechnicalIndicators + Send + Sync>), // <= cast box<struct> as box<trait>
                        SMA_IND => Ok(Box::new(SmaTac::new(candles, period))
                            as Box<dyn TechnicalIndicators + Send + Sync>),
                        other => Err(eyre!("Not found indicator {}!", other)),
                    };
                result
            });
        let tac = match result {
            Ok(tac) => tac,
            Err(e) => return Err(eyre!("{}", e)),
        };
        Ok(tac.main_indicator())
    }

    fn macd(
        &mut self,
        now: DateTime<Utc>,
        candles: &[Candle],
        ind_name: &str,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> eyre::Result<&Indicator> {
        // Try to reuse the same tiple macd/signal/divergence
        self.macds_opt = self.macds_opt.take().filter(|e| {
            e.0 == now && e.1 == fast_period && e.2 == slow_period && e.3 == signal_period
        });
        let macd = self.macds_opt.get_or_insert_with(|| {
            (
                now,
                fast_period,
                slow_period,
                signal_period,
                MacdTac::new(candles, fast_period, slow_period, signal_period),
            )
        });
        let result = macd
            .4
            .indicators
            .get(ind_name)
            .ok_or_else(|| -> eyre::Error { eyre!("Not found indicator {}!", ind_name) });
        result
    }

    pub fn indicator(
        &mut self,
        now: DateTime<Utc>,
        candles: &[Candle],
        i_type: &IndicatorType,
    ) -> eyre::Result<&Indicator> {
        let ind = match i_type {
            IndicatorType::Macd(fast_period, slow_period, signal_period) => self.macd(
                now,
                candles,
                MACD_IND,
                *fast_period,
                *slow_period,
                *signal_period,
            )?,
            IndicatorType::MacdSignal(fast_period, slow_period, signal_period) => self.macd(
                now,
                candles,
                MACD_SIG_IND,
                *fast_period,
                *slow_period,
                *signal_period,
            )?,
            IndicatorType::MacdDivergence(fast_period, slow_period, signal_period) => self.macd(
                now,
                candles,
                MACD_DIV_IND,
                *fast_period,
                *slow_period,
                *signal_period,
            )?,
            IndicatorType::Ema(period) => self.tac_indicator(candles, EMA_IND, *period)?,
            IndicatorType::Sma(period) => self.tac_indicator(candles, SMA_IND, *period)?,
            //IndicatorType::TopBottom(period) => self.tac_indicator(candles, TOP_BOTTOM_IND, *period)?,
        };
        Ok(ind)
    }
}