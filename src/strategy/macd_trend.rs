use super::{trend::Trend, trend_prov_factory::TrendProviderFactory, trend_provider::TrendProvider};
use crate::{
    model::candle::Candle,
    technicals::{ind_provider::IndicatorProvider, ind_type::IndicatorType},
};

/// setup

/// transfer 1000 USD
/// buy 500 USD

pub struct MacdTrend<'a> {
    indicator_provider: IndicatorProvider<'a>,
}

impl<'a> MacdTrend<'a> {
    pub fn new(indicator_provider: IndicatorProvider<'a>) -> Self {
        Self { indicator_provider }
    }
}

impl<'a> TrendProvider<'a> for MacdTrend<'a> {
    fn trend(&mut self, candles: &'a [&Candle]) -> Trend {
        let _mcad = self.indicator_provider.indicator(candles, &IndicatorType::Macd(34, 72, 17)).unwrap();
        todo!()
    }
}

// impl<'a> TrendProviderFactory<'a, MacdTrend<'a>> for MacdTrend<'a> {
//     fn create(indicator_provider: IndicatorProvider<'a>) -> MacdTrend<'a> {
//         MacdTrend::new(indicator_provider)
//     }
// }
