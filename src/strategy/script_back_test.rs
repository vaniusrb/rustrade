use super::{
    trader_register::TradeOperation,
    trend::{callback_trend_provider::CallBackTrendProvider, trend_provider::TrendProvider},
};
use crate::application::plot_selection::PlotterSelection;
use crate::model::price::Price;
use crate::model::quantity;
use crate::strategy::flow_register::FlowRegister;
use crate::strategy::operation::Operation;
use crate::strategy::position::Position;
use crate::strategy::script_fns::*;
use crate::strategy::singleton_context::ContextSingleton;
use crate::strategy::singleton_engine::EngineSingleton;
use crate::strategy::singleton_position::PositionSingleton;
use crate::strategy::trader_register::TraderRegister;
use crate::tac_plotters::indicator_plotter::PlotterIndicatorContext;
use crate::tac_plotters::trading_plotter::TradingPlotter;
use crate::{
    application::{app::Application, candles_provider::CandlesProvider},
    strategy::back_test_runner::TraderFactory,
};
use eyre::eyre;
use ifmt::iformat;
use log::info;
use quantity::Quantity;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use rust_decimal_macros::dec;
use std::{path::Path, time::Instant};

const FN_BUY: &str = "buy";

/// Run script back test
pub fn run_script<P: AsRef<Path>>(app: &mut Application, script_file: P) -> eyre::Result<Vec<TradeOperation>> {
    let start = Instant::now();
    info!("Initializing back test...");

    // Create engine script and register functions
    EngineSingleton::install(script_file)?;

    // Load candles from selection
    app.candles_provider
        .set_candles_selection(app.selection.candles_selection.clone());
    let candles = app.candles_provider.candles()?;

    // Create trend provider with call back
    let callback_trend_provider = CallBackTrendProvider::from(|position, trade_context_provider| {
        // Set current static trade_context_provider and position
        ContextSingleton::set_current(trade_context_provider);
        PositionSingleton::set_current(position);

        // Get engine to run script
        let engine_arc = EngineSingleton::current();
        let (engine, scope, ast) = &engine_arc.engine_scope.as_ref().unwrap();
        // Run script
        let quantity: f64 = engine.call_fn(&mut scope.clone(), &ast, FN_BUY, ()).unwrap();
        // Check if have to do nothing, buy or sell asset
        let result = if quantity > 0. {
            Some(Operation::Buy(Quantity(Decimal::from_f64(quantity).unwrap())))
        } else if quantity < 0. {
            Some(Operation::Sell(Quantity(Decimal::from_f64(quantity * -1.).unwrap())))
        } else {
            None
        };
        Ok(result)
    });

    let flow_register = FlowRegister::new();

    // Initial position
    let price = Price(candles.first().ok_or_else(|| eyre!("First candle not found!"))?.open);
    let position = Position::from_fiat(flow_register, dec!(1000), price);

    let trader_register = TraderRegister::from(position);

    // TODO Probably candles_provider can be within something like a ContextProvider, then can provides date_time and price

    // Create trader from trend provider
    let trader_factory = TraderFactory::from(app.selection.candles_selection.clone(), app.candles_provider.clone());
    let trend_provider: Box<dyn TrendProvider + Send + Sync> = Box::new(callback_trend_provider);
    let mut trader = trader_factory.create_trader(trend_provider, trader_register);

    // Run trader from candles, this invoke callback_trend_provider for each candle (run script)
    candles.iter().for_each(|c| {
        trader.check(c.close_time, Price(c.close)).unwrap();
    });

    info!("{}", iformat!("Finished back test, elapsed: {start.elapsed():?}"));

    // Get realized trades
    let trades = trader.trades();

    {
        // Create default plotter selection
        app.selection.image_name = "out/back_test.png".into();
        let mut plotter_selection =
            PlotterSelection::from(app.selection.clone(), app.candles_provider.clone_provider());

        // Add plotter for trading marks
        let trading_plotter = TradingPlotter::new(&trades);
        let plotters = vec![Box::new(trading_plotter) as Box<dyn PlotterIndicatorContext>];
        plotters.into_iter().for_each(|p| plotter_selection.push_plotter_ind(p));

        // Plot image
        plotter_selection.plot()?;
    }

    Ok(trades)
}
