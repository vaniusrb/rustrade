use crate::model::candle::Candle;

pub trait CandlesProvider {
    fn candles(&mut self) -> eyre::Result<Vec<Candle>>;
    fn clone_provider(&self) -> Box<dyn CandlesProvider>;
}
