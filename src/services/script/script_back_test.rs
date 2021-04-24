use crate::model::position::Position;
use crate::repository::flow_repository::FlowRepository;
use crate::repository::position_repository::PositionRepository;
use crate::services::provider::candles_provider::CandlesProvider;
use crate::services::script::position_register::PositionRegister;
use crate::services::script::script_trend_provider::ScriptTrendProvider;
use crate::services::script::singleton_engine::EngineSingleton;
use crate::services::tec_plotter::plot_selection::PlotterSelection;
use crate::services::tec_plotter::plotter_indicator_context::PlotterIndicatorContext;
use crate::services::tec_plotter::trading_plotter::TradingPlotter;
use crate::services::trading::flow_register::FlowRegister;
use crate::services::trading::trade_operation::TradeOperation;
use crate::services::trading::trader_factory::TraderFactory;
use crate::services::trading::trader_register::TraderRegister;
use crate::{app::Application, model::price::Price};
use colored::Colorize;
use eyre::eyre;
use ifmt::iformat;
use log::info;
use rust_decimal_macros::dec;
use sqlx::PgPool;
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

    // Create trader from trend provider
    let trader_factory = TraderFactory::from(
        app.selection.candles_selection,
        app.candles_provider.clone(),
    );

    let script_trend_provider = ScriptTrendProvider::new();

    let mut trader = trader_factory.create_trader(script_trend_provider, trader_register);

    // Run trader from candles, this invoke script_trend_provider.trend()
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_to_description_test() {
        assert_eq!(path_to_description("/~/scripts/macd.rhai"), "macd");
    }
}
