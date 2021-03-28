use super::{
    trader_register::TradeOperation,
    trend::{callback_trend_provider::CallBackTrendProvider, trend_provider::TrendProvider},
};
use crate::application::plot_selection::plot_selection;
use crate::strategy::trader_register::Position;
use crate::strategy::trader_register::TraderRegister;
use crate::tac_plotters::indicator_plotter::PlotterIndicatorContext;
use crate::tac_plotters::trading_plotter::TradingPlotter;
use crate::{
    application::{app::Application, candles_provider::CandlesProvider},
    strategy::{back_test_runner::TraderFactory, trade_context_provider::TradeContextProvider, trend_enum::Trend},
    technicals::ind_type::IndicatorType,
};
use ifmt::iformat;
use log::info;
use rhai::{Engine, RegisterFn, Scope, AST};
use rust_decimal_macros::dec;
use std::{
    fs,
    path::Path,
    sync::{Arc, RwLock},
    time::Instant,
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
    pub fn set_current(trade_context_provider: TradeContextProvider) {
        Self {
            trade_context_provider: Some(trade_context_provider),
        }
        .make_current();
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
    pub fn set_current(engine_scope: (Engine, Scope<'static>, AST)) {
        Self {
            engine_scope: Some(engine_scope),
        }
        .make_current();
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

fn macd_divergence(min: i64, a: i64, b: i64, c: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider.as_ref().unwrap();
    trade_context_provider
        .indicator(
            min as u32,
            &IndicatorType::MacdDivergence(a as usize, b as usize, c as usize),
        )
        .unwrap()
        .value()
        .unwrap()
}

fn ema(min: i64, a: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider.as_ref().unwrap();
    trade_context_provider
        .indicator(min as u32, &IndicatorType::Ema(a as usize))
        .unwrap()
        .value()
        .unwrap()
}

fn sma(min: i64, a: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider.as_ref().unwrap();
    trade_context_provider
        .indicator(min as u32, &IndicatorType::Sma(a as usize))
        .unwrap()
        .value()
        .unwrap()
}

// TODO current ticket price
// TODO current ticket bought/sold
// TODO current date/time
// TODO current position bought/sold
// TODO last price bought/sold
// TODO last percent bought/sold
//

const FN_BUY: &str = "buy";

/// Run script back test
pub fn run_script<P: AsRef<Path>>(app: &mut Application, file: P) -> anyhow::Result<Vec<TradeOperation>> {
    let start = Instant::now();
    info!("Initializing back test...");

    // Create engine script and register functions
    let mut engine = Engine::new();
    engine.register_fn("ema", ema);
    engine.register_fn("sma", sma);
    engine.register_fn("macd", macd);
    engine.register_fn("macd_signal", macd_signal);
    engine.register_fn("macd_divergence", macd_divergence);

    // Load script file and compile AST
    let script_content = fs::read_to_string(file)?;
    let ast = engine.compile(&script_content)?;
    // Define script engine singleton
    EngineSingleton::set_current((engine, Scope::new(), ast));

    // Create trader factory
    let trader_factory = TraderFactory::from(app.selection.candles_selection.clone(), app.candles_provider.clone());
    app.candles_provider
        .set_candles_selection(app.selection.candles_selection.clone());

    // Load candles from selection
    let candles = app.candles_provider.candles()?;

    // Create trend provider with call back
    let callback_trend_provider = CallBackTrendProvider::from(|trade_context_provider| {
        // Set current static trade_context_provider
        ContextSingleton::set_current(trade_context_provider);
        // Get engine and run script
        let engine_arc = EngineSingleton::current();
        let (engine, scope, ast) = &engine_arc.engine_scope.as_ref().unwrap();
        let result: bool = engine.call_fn(&mut scope.clone(), &ast, FN_BUY, ()).unwrap();
        // Return trend
        Ok(if result { Trend::Bought } else { Trend::Sold })
    });

    let position = Position::from_usd(dec!(1000));

    let trader_register = TraderRegister::from(position);

    // Create trader from trend provider
    let trend_provider: Box<dyn TrendProvider + Send + Sync> = Box::new(callback_trend_provider);
    let mut trader = trader_factory.create_trader(trend_provider, trader_register);

    // Run trader from candles
    candles.iter().for_each(|c| {
        trader.check(c.close_time, c.close).unwrap();
    });

    let trades = trader.trades();

    let trading_plotter = TradingPlotter::new(&trades);

    let plotters = vec![Box::new(trading_plotter) as Box<dyn PlotterIndicatorContext>];

    app.selection.image_name = "out/back_test.png".into();

    plot_selection(app.selection.clone(), app.candles_provider.clone_provider(), plotters)?;

    info!("{}", iformat!("Finished back test, elapsed: {start.elapsed():?}"));

    Ok(trades)
}
