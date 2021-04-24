use super::candles_provider::CandlesProvider;
use super::candles_provider_buffer::CandlesProviderBuffer;
use crate::config::candles_selection::CandlesSelection;
use crate::model::candle::Candle;

pub struct CandlesProviderSelection {
    candles_provider: CandlesProviderBuffer,
    candles_selection: CandlesSelection,
}

impl<'a> CandlesProviderSelection {
    pub fn new(
        candles_provider: CandlesProviderBuffer,
        candles_selection: CandlesSelection,
    ) -> Self {
        Self {
            candles_provider,
            candles_selection,
        }
    }

    pub fn candles_selection(&self) -> CandlesSelection {
        self.candles_selection
    }
}

impl<'a> CandlesProvider for CandlesProviderSelection {
    fn candles(&mut self) -> eyre::Result<Vec<Candle>> {
        // TODO HERE SHOULD FILTER?
        self.candles_provider
            .set_candles_selection(self.candles_selection);
        self.candles_provider.candles()
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        Box::new(Self::new(
            self.candles_provider.clone(),
            self.candles_selection,
        ))
    }
}
