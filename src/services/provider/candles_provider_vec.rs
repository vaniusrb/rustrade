use super::candles_provider::CandlesProvider;
use crate::model::candle::Candle;

pub struct CandlesProviderVec {
    candles: Vec<Candle>,
}

impl<'a> CandlesProviderVec {
    pub fn new(candles: &'a [Candle], last_n: usize) -> Self {
        let start = (candles.len() - last_n).max(0);
        Self {
            candles: candles[start..candles.len()].to_vec(),
        }
    }
}

impl CandlesProvider for CandlesProviderVec {
    fn candles(&mut self) -> eyre::Result<Vec<Candle>> {
        Ok(self.candles.to_vec())
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        Box::new(Self::new(self.candles.as_ref(), 0))
    }
}
