use super::script_fns::*;
use rhai::{Engine, Scope, AST};
use std::{
    fs,
    path::Path,
    sync::{Arc, RwLock},
};

/// Singleton for engine script
#[derive(Default)]
pub struct EngineSingleton {
    pub engine_scope: Option<(Engine, Scope<'static>, AST)>,
}

impl EngineSingleton {
    pub fn current() -> Arc<EngineSingleton> {
        CURRENT_ENGINE.with(|c| c.read().unwrap().clone())
    }

    pub fn make_current(self) {
        CURRENT_ENGINE.with(|c| *c.write().unwrap() = Arc::new(self))
    }

    pub fn set_current(engine_scope: (Engine, Scope<'static>, AST)) {
        Self {
            engine_scope: Some(engine_scope),
        }
        .make_current();
    }

    /// Create engine script and register functions
    pub fn install<P: AsRef<Path>>(script_file: P) -> eyre::Result<()> {
        // Create engine script and register functions
        let mut engine = Engine::new();
        // Current context
        engine.register_fn("rsi", rsi);
        engine.register_fn("price", price);
        engine.register_fn("ema", ema);
        engine.register_fn("sma", sma);
        engine.register_fn("macd", macd);
        engine.register_fn("macd_signal", macd_signal);
        engine.register_fn("macd_divergence", macd_divergence);
        // Conversion functions
        engine.register_fn("fiat_to_asset", fiat_to_asset);
        engine.register_fn("asset_to_fiat", asset_to_fiat);
        // Current position
        engine.register_fn("balance_fiat", balance_fiat);
        engine.register_fn("balance_asset", balance_asset);
        engine.register_fn("is_bought", is_bought);
        engine.register_fn("is_sold", is_sold);
        engine.register_fn("gain_perc", gain_perc);

        // Operations
        engine.register_fn("sell", sell);
        engine.register_fn("buy", buy);

        // Debugging
        engine.register_fn("log", log);

        // Load script file and compile AST
        let script_content = fs::read_to_string(script_file)?;
        let ast = engine.compile(&script_content)?;
        // Define script engine singleton
        EngineSingleton::set_current((engine, Scope::new(), ast));

        Ok(())
    }
}

thread_local! {
    static CURRENT_ENGINE: RwLock<Arc<EngineSingleton>> = RwLock::new(Default::default());
}
