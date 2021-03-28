use super::trend::{callback_trend_provider::CallBackTrendProvider, trend_provider::TrendProvider};
use crate::{
    application::{app::Application, candles_provider::CandlesProvider},
    strategy::{back_test_runner::TraderFactory, trade_context_provider::TradeContextProvider, trend_enum::Trend},
    technicals::ind_type::IndicatorType,
};
use rhai::{Engine, RegisterFn, Scope, AST};
use std::{
    fs,
    path::Path,
    sync::{Arc, RwLock},
};

/// Singleton for trade context
#[derive(Default)]
struct ContextSingleton {
    pub trade_context_provider: Option<TradeContextProvider>,
}

impl ContextSingleton {
    pub fn current() -> Arc<ContextSingleton> {
        CURRENT_CONTEXT.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_CONTEXT.with(|c| *c.write().unwrap() = Arc::new(self))
    }
}

thread_local! {
    static CURRENT_CONTEXT: RwLock<Arc<ContextSingleton>> = RwLock::new(Default::default());
}

/// Singleton for engine script
#[derive(Default)]
struct EngineSingleton {
    pub engine_scope: Option<(Engine, Scope<'static>, AST)>,
}

impl EngineSingleton {
    pub fn current() -> Arc<EngineSingleton> {
        CURRENT_ENGINE.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_ENGINE.with(|c| *c.write().unwrap() = Arc::new(self))
    }
}

thread_local! {
    static CURRENT_ENGINE: RwLock<Arc<EngineSingleton>> = RwLock::new(Default::default());
}

fn macd(min: i64, a: i64, b: i64, c: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider.as_ref().unwrap();
    trade_context_provider
        .indicator(min as u32, &IndicatorType::Macd(a as usize, b as usize, c as usize))
        .unwrap()
        .value()
        .unwrap()
}

fn macd_signal(min: i64, a: i64, b: i64, c: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider.as_ref().unwrap();
    trade_context_provider
        .indicator(
            min as u32,
            &IndicatorType::MacdSignal(a as usize, b as usize, c as usize),
        )
        .unwrap()
        .value()
        .unwrap()
}

/// Run script back test
pub fn run_script<P: AsRef<Path>>(app: &mut Application, file: P) -> anyhow::Result<()> {
    // Create engine script and register functions
    let mut engine = Engine::new();
    engine.register_fn("macd", macd);
    engine.register_fn("macd_signal", macd_signal);

    // Load script file and compile AST
    let script_content = fs::read_to_string(file)?;
    let ast = engine.compile(&script_content)?;
    let scope = Scope::new();
    // Define script engine singleton
    EngineSingleton {
        engine_scope: Some((engine, scope, ast)),
    }
    .make_current();

    // Create trader factory
    let trader_factory = TraderFactory::new(app.selection.candles_selection.clone(), app.candles_provider.clone());
    app.candles_provider
        .set_candles_selection(app.selection.candles_selection.clone());

    // Load candles from selection
    let candles = app.candles_provider.candles()?;

    // Create trend provider with call back
    let callback_trend_provider = CallBackTrendProvider::from(|trade_context_provider| {
        // Set current static trade_context_provider
        ContextSingleton {
            trade_context_provider: Some(trade_context_provider),
        }
        .make_current();

        // Get engine and run script
        let engine_arc = EngineSingleton::current();
        let (engine, scope, ast) = &engine_arc.engine_scope.as_ref().unwrap();
        let result: bool = engine.call_fn(&mut scope.clone(), &ast, "buy", ()).unwrap();
        // Return trend
        Ok(if result { Trend::Bought } else { Trend::Sold })
    });

    // Create trader from trend provider
    let trend_provider: Box<dyn TrendProvider + Send + Sync> = Box::new(callback_trend_provider);
    let mut trader = trader_factory.create_trader(trend_provider);

    // Run trader from candles
    candles.iter().for_each(|c| {
        trader.check(c.close_time, c.close).unwrap();
        //trader.trades();
    });

    Ok(())
}
