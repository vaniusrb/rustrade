use super::candles_provider::CandlesProvider;
use crate::model::candle::Candle;

pub struct CandlesProviderClosure<F>
where
    F: FnMut() -> eyre::Result<Vec<Candle>>,
{
    call_back: F,
}

impl<'a, F> CandlesProviderClosure<F>
where
    F: FnMut() -> eyre::Result<Vec<Candle>>,
{
    pub fn new(call_back: F) -> Self
    where
        F: FnMut() -> eyre::Result<Vec<Candle>>,
    {
        Self { call_back }
    }
}

impl<'a, F> CandlesProvider for CandlesProviderClosure<F>
where
    F: FnMut() -> eyre::Result<Vec<Candle>>,
{
    fn candles(&mut self) -> eyre::Result<Vec<Candle>> {
        (self.call_back)()
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        todo!()
    }
}
