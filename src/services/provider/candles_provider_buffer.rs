use super::candles_provider::CandlesProvider;
use super::candles_provider_buffer_singleton::CandlesProviderBufferSingleton;
use crate::config::candles_selection::CandlesSelection;
use crate::model::candle::Candle;
use eyre::eyre;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone)]
pub struct CandlesProviderBuffer {
    candles_provider_singleton: Arc<RwLock<CandlesProviderBufferSingleton>>,
    candles_selection_opt: Option<CandlesSelection>,
}

impl CandlesProviderBuffer {
    pub fn new(candles_provider_singleton: Arc<RwLock<CandlesProviderBufferSingleton>>) -> Self {
        Self {
            candles_provider_singleton,
            candles_selection_opt: None,
        }
    }
    pub fn set_candles_selection(&mut self, candles_selection: CandlesSelection) {
        self.candles_selection_opt = Some(candles_selection);
    }
}

impl CandlesProvider for CandlesProviderBuffer {
    fn candles(&mut self) -> eyre::Result<Vec<Candle>> {
        let candles_selection = self
            .candles_selection_opt
            .as_ref()
            .cloned()
            .ok_or_else(|| -> eyre::Error { eyre!("candles_selection not defined!") })?;

        let m = &*self.candles_provider_singleton;

        let mut c = m.write().unwrap();

        c.candles(candles_selection)
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        let candles_provider = Self {
            candles_provider_singleton: self.candles_provider_singleton.clone(),
            candles_selection_opt: self.candles_selection_opt,
        };
        Box::new(candles_provider)
    }
}
