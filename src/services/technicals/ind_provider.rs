use super::indicator::Indicator;
use super::min_max_tec::MinMaxTec;
use super::min_max_tec::IND_MIN_MAX;
use super::{
    ema_tec::{EmaTec, IND_EMA},
    ind_type::IndicatorType,
    macd_tec::{MacdTec, IND_MACD, IND_MACD_DIV, IND_MACD_SIG},
    rsi_tec::{RsiTec, IND_RSI},
    sma_tec::{SmaTec, IND_SMA},
    technical::TechnicalIndicators,
};
use crate::model::candle::Candle;
use chrono::{DateTime, Utc};
use eyre::eyre;
use std::collections::HashMap;

pub struct IndicatorProvider {
    macds_tec_opt: Option<(DateTime<Utc>, usize, usize, usize, MacdTec)>,
    min_max_tec_opt: Option<(DateTime<Utc>, usize, MinMaxTec)>,
    tec_indicators:
        HashMap<(String, usize), eyre::Result<Box<dyn TechnicalIndicators + Send + Sync>>>, // <= to allow trait with different lifetime
}

impl IndicatorProvider {
    pub fn new() -> Self {
        Self {
            macds_tec_opt: None,
            min_max_tec_opt: None,
            tec_indicators: HashMap::new(),
        }
    }

    fn tec_indicator(
        &mut self,
        candles: &[Candle],
        ind_name: &str,
        period: usize,
    ) -> eyre::Result<&dyn Indicator> {
        self.tec_indicators.clear();
        // TODO I shouldn't store Indicator cache, or use "now" like a key
        let result: &mut eyre::Result<Box<dyn TechnicalIndicators + Send + Sync>> = self
            .tec_indicators
            .entry((ind_name.to_string(), period))
            .or_insert_with(|| {
                let result: eyre::Result<Box<dyn TechnicalIndicators + Send + Sync>> =
                    // TODO here should use enum instead constant
                    match ind_name {
                        IND_EMA => Ok(Box::new(EmaTec::new(candles, period))
                            as Box<dyn TechnicalIndicators + Send + Sync>),
                        IND_SMA => Ok(Box::new(SmaTec::new(candles, period))
                            as Box<dyn TechnicalIndicators + Send + Sync>),
                        IND_RSI => Ok(Box::new(RsiTec::new(candles, period))
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

    fn min_max_indicator(
        &mut self,
        now: DateTime<Utc>,
        candles: &[Candle],
        ind_name: &str,
        period: usize,
    ) -> eyre::Result<&dyn Indicator> {
        // Try to reuse the same triple macd/signal/divergence
        self.min_max_tec_opt = self
            .min_max_tec_opt
            .take()
            .filter(|e| e.0 == now && e.1 == period);
        let macd = self
            .min_max_tec_opt
            .get_or_insert_with(|| (now, period, MinMaxTec::new(candles, period)));
        let result = macd
            .2
            .get_indicator(ind_name)
            .ok_or_else(|| -> eyre::Error { eyre!("Not found indicator {}!", ind_name) });
        result
    }

    fn macd_indicator(
        &mut self,
        now: DateTime<Utc>,
        candles: &[Candle],
        ind_name: &str,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> eyre::Result<&dyn Indicator> {
        // Try to reuse the same triple macd/signal/divergence
        self.macds_tec_opt = self.macds_tec_opt.take().filter(|e| {
            e.0 == now && e.1 == fast_period && e.2 == slow_period && e.3 == signal_period
        });
        let macd = self.macds_tec_opt.get_or_insert_with(|| {
            (
                now,
                fast_period,
                slow_period,
                signal_period,
                MacdTec::new(candles, fast_period, slow_period, signal_period),
            )
        });
        let result = macd
            .4
            .get_indicator(ind_name)
            .ok_or_else(|| -> eyre::Error { eyre!("Not found indicator {}!", ind_name) });
        result
    }

    pub fn indicator(
        &mut self,
        now: DateTime<Utc>,
        candles: &[Candle],
        indicator_type: &IndicatorType,
    ) -> eyre::Result<&dyn Indicator> {
        let indicator = match indicator_type {
            IndicatorType::MinMax(period) => {
                self.min_max_indicator(now, candles, IND_MIN_MAX, *period)?
            }

            IndicatorType::Macd(fast_period, slow_period, signal_period) => self.macd_indicator(
                now,
                candles,
                IND_MACD,
                *fast_period,
                *slow_period,
                *signal_period,
            )?,
            IndicatorType::MacdSignal(fast_period, slow_period, signal_period) => self
                .macd_indicator(
                    now,
                    candles,
                    IND_MACD_SIG,
                    *fast_period,
                    *slow_period,
                    *signal_period,
                )?,
            IndicatorType::MacdDivergence(fast_period, slow_period, signal_period) => self
                .macd_indicator(
                    now,
                    candles,
                    IND_MACD_DIV,
                    *fast_period,
                    *slow_period,
                    *signal_period,
                )?,
            IndicatorType::Ema(period) => self.tec_indicator(candles, IND_EMA, *period)?,
            IndicatorType::Sma(period) => self.tec_indicator(candles, IND_SMA, *period)?,
            IndicatorType::Rsi(period) => self.tec_indicator(candles, IND_RSI, *period)?,
            //IndicatorType::TopBottom(period) => self.tec_indicator(candles, TOP_BOTTOM_IND, *period)?,
        };
        Ok(indicator)
    }
}

impl Default for IndicatorProvider {
    fn default() -> Self {
        Self::new()
    }
}
