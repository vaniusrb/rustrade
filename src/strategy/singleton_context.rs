use std::sync::{Arc, RwLock};

use super::trade_context_provider::TradeContextProvider;

/// Singleton for trade context
#[derive(Default)]
pub struct ContextSingleton {
    pub trade_context_provider_opt: Option<TradeContextProvider>,
}

impl ContextSingleton {
    pub fn current() -> Arc<ContextSingleton> {
        CURRENT_CONTEXT.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_CONTEXT.with(|c| *c.write().unwrap() = Arc::new(self))
    }
    pub fn set_current(trade_context_provider: TradeContextProvider) {
        Self {
            trade_context_provider_opt: Some(trade_context_provider),
        }
        .make_current();
    }
}

thread_local! {
    static CURRENT_CONTEXT: RwLock<Arc<ContextSingleton>> = RwLock::new(Default::default());
}
