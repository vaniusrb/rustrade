use crate::model::operation::Operation;
use crate::model::position::Position;
use crate::model::quantity;
use crate::repository::flow_repository::FlowRepository;
use crate::repository::position_repository::PositionRepository;
use crate::services::provider::candles_provider::CandlesProvider;
use crate::services::script::position_register::PositionRegister;
use crate::services::script::singleton_context::ContextSingleton;
use crate::services::script::singleton_engine::EngineSingleton;
use crate::services::script::singleton_position::PositionRegisterSingleton;
use crate::services::tec_plotter::plot_selection::PlotterSelection;
use crate::services::tec_plotter::plotter_indicator_context::PlotterIndicatorContext;
use crate::services::tec_plotter::trading_plotter::TradingPlotter;
use crate::services::trader::flow_register::FlowRegister;
use crate::services::trader::running_script_state::TrendState;
use crate::services::trader::trade_operation::TradeOperation;
use crate::services::trader::trader_factory::TraderFactory;
use crate::services::trader::trader_register::TraderRegister;
use crate::services::trader::trend::callback_trend_provider::CallBackTrendProvider;
use crate::services::trader::trend::trend_direction::TrendDirection;
use crate::services::trader::trend::trend_provider::TrendProvider;
use crate::{app::Application, model::price::Price};
use colored::Colorize;
use eyre::eyre;
use ifmt::iformat;
use log::info;
use quantity::Quantity;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use rust_decimal_macros::dec;
use sqlx::PgPool;
use std::cmp::Ordering;
use std::{
    path::Path,
    sync::{Arc, RwLock},
    time::Instant,
};

fn path_to_description<P: AsRef<Path>>(path: P) -> String {
    let script_file_path = path.as_ref();
    script_file_path
        .with_extension("")
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

/// Run script back test
pub fn run_script<P: AsRef<Path>>(
    pool: Arc<RwLock<PgPool>>,
    app: &mut Application,
    script_file: P,
) -> eyre::Result<Vec<TradeOperation>> {
    let start = Instant::now();
    info!("Initializing back test...");

    // Create engine script and register functions
    EngineSingleton::install(&script_file)?;

    // Load candles from selection
    app.candles_provider
        .set_candles_selection(app.selection.candles_selection);
    let candles = app.candles_provider.candles()?;

    // Create trend provider with call back
    let callback_trend_provider =
        CallBackTrendProvider::from(|position_register, trade_context_provider| {
            let changed_trend = trade_context_provider.changed_trend();

            // Set current static trade_context_provider and position to script functions can read this
            ContextSingleton::set_current(trade_context_provider);
            PositionRegisterSingleton::set_current(position_register);

            // Get engine to run script
            let engine_arc = EngineSingleton::current();
            let (engine, scope, ast) = &engine_arc.engine_scope.as_ref().unwrap();

            // Retrieving trend direction
            let trend_direction = {
                let trend: i64 = engine
                    .call_fn(&mut scope.clone(), &ast, "trend", ())
                    .unwrap();
                match trend.cmp(&0) {
                    Ordering::Greater => TrendDirection::Buy,
                    Ordering::Less => TrendDirection::Sell,
                    Ordering::Equal => TrendDirection::None,
                }
            };

            // Retrieving quantity to buy or sell
            let operation_opt = {
                let quantity: f64 = if let Some(trend_direction) = changed_trend {
                    let trend = match trend_direction {
                        TrendDirection::Buy => 1,
                        TrendDirection::Sell => -1,
                        TrendDirection::None => 0,
                    };
                    engine
                        .call_fn(&mut scope.clone(), &ast, "change_trend", (trend as i64,))
                        .unwrap()
                } else {
                    engine.call_fn(&mut scope.clone(), &ast, "run", ()).unwrap()
                };
                quantity_to_operation_opt(quantity)
            };

            Ok(TrendState {
                trend_direction,
                operation_opt,
            })
        });

    let flow_repository = FlowRepository::new(pool.clone());
    let flow_register = FlowRegister::new(flow_repository.clone());

    // Initial position
    let _first_price = Price(
        candles
            .first()
            .ok_or_else(|| eyre!("First candle not found!"))?
            .open,
    );

    let position_description = path_to_description(&script_file);
    let position_repository = PositionRepository::new(pool);

    let position_opt = position_repository.position_by_description(&position_description);
    if let Some(position) = position_opt {
        flow_repository.delete_flows_from_position(position.id);
        position_repository.delete_position(position.id);
    }

    let mut position = Position::from_fiat(&position_description, dec!(1000));
    position_repository.insert_position(&mut position)?;

    let position_register = PositionRegister::new(position, flow_register);

    let trader_register = TraderRegister::from(position_register);

    // TODO Probably candles_provider can be within something like a ContextProvider, then can provides date_time and price

    // Create trader from trend provider
    let trader_factory = TraderFactory::from(
        app.selection.candles_selection,
        app.candles_provider.clone(),
    );
    let trend_provider: Box<dyn TrendProvider + Send + Sync> = Box::new(callback_trend_provider);
    let mut trader = trader_factory.create_trader(trend_provider, trader_register);

    // Run trader from candles, this invoke callback_trend_provider for each candle (run script)
    candles.iter().for_each(|c| {
        trader.check(c.close_time, Price(c.close)).unwrap();
    });

    info!(
        "{}",
        iformat!(
            "Finished back test, total read candles: {candles.len()} elapsed: {start.elapsed():?}"
        )
        .bright_cyan()
    );

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
        plotters
            .into_iter()
            .for_each(|p| plotter_selection.push_plotter_ind(p));

        // Plot image
        plotter_selection.plot()?;
    }

    Ok(trades)
}

fn quantity_to_operation_opt(quantity: f64) -> Option<Operation> {
    if quantity > 0. {
        Some(Operation::Buy(Quantity(
            Decimal::from_f64(quantity).unwrap(),
        )))
    } else if quantity < 0. {
        Some(Operation::Sell(Quantity(
            Decimal::from_f64(quantity * -1.).unwrap(),
        )))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_to_description_test() {
        assert_eq!(path_to_description("/~/scripts/macd.rhai"), "macd");
    }
}
