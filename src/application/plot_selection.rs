use super::candles_provider::CandlesProvider;
use crate::technicals::top_bottom_tac::TopBottomTac;
use crate::{
    config::selection::Selection,
    tac_plotters::{
        candles_plotter::CandlePlotter, indicator_plotter::PlotterIndicatorContext,
        line_ind_plotter::LineIndicatorPlotter, macd_plotter::MacdPlotter, plotter::Plotter,
        top_bottom_plotter::TopBottomPlotter,
    },
    technicals::technical::TechnicalIndicators,
    technicals::{ema_tac::EmaTac, macd::macd_tac::MacdTac},
};
use colored::Colorize;
use ifmt::iformat;
use log::info;
use plotters::style::RGBColor;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Instant;

pub struct PlotterSelecion {
    selection: Selection,
    candles_provider: Box<dyn CandlesProvider>,
}

impl PlotterSelecion {
    pub fn add_plotter_ind(&mut self) {}

    pub fn plot(&self) {}
}

pub fn plot_selection<'a>(
    selection: Selection,
    mut candles_provider: Box<dyn CandlesProvider>,
    additional_plotters: Vec<Box<dyn PlotterIndicatorContext + 'a>>,
) -> anyhow::Result<()> {
    let total_start = Instant::now();

    let candles_provider_clone = candles_provider.clone_provider();
    let candles = candles_provider.candles()?;

    let start_time = selection.candles_selection.start_time;
    let end_time = selection.candles_selection.end_time;

    // TODO Is possible there is any candle out of range? Is this necessary?
    let candles = candles
        .par_iter()
        .filter(|c| c.open_time >= start_time && c.open_time <= end_time)
        .collect::<Vec<_>>();

    info!(
        "Plotting selection {:?} {:?} candles.len {} image {}",
        selection.candles_selection.start_time,
        selection.candles_selection.end_time,
        candles.len(),
        selection.image_name.green()
    );

    // Default technicals
    let macd_tac = MacdTac::new(candles_provider_clone.clone_provider(), 34, 72, 17);
    let ema_short_tac = EmaTac::new(candles_provider_clone.clone_provider(), 17);
    let ema_long_tac = EmaTac::new(candles_provider_clone.clone_provider(), 72);
    let mut top_bottom_tac = TopBottomTac::new(candles_provider_clone, 7);
    let top_bottoms = top_bottom_tac.top_bottoms()?;

    // Create plotter object
    let mut plotter = Plotter::new(selection.clone());

    // ema 17 = purple
    let short_purple = RGBColor(128, 0, 128);
    // ema 72 = orange
    let long_orange = RGBColor(255, 165, 0);
    // Upper indicators plotters
    let candle_plotter = CandlePlotter::new(&candles);
    let ema_short_plotter = LineIndicatorPlotter::new(ema_short_tac.main_indicator(), short_purple);
    let ema_long_plotter = LineIndicatorPlotter::new(ema_long_tac.main_indicator(), long_orange);
    let topbottom_plotter = TopBottomPlotter::new(&top_bottoms);
    plotter.add_plotter_upper_ind(&candle_plotter);
    plotter.add_plotter_upper_ind(&topbottom_plotter);
    plotter.add_plotter_upper_ind(&ema_short_plotter);
    plotter.add_plotter_upper_ind(&ema_long_plotter);

    // Custom indicators
    additional_plotters
        .iter()
        .for_each(|p| plotter.add_plotter_upper_ind(&**p));

    // Lower indicators plotters
    let macd_plotter = MacdPlotter::new(&macd_tac);
    plotter.add_plotter_ind(&macd_plotter);

    let start = Instant::now();

    plotter.plot(&selection.image_name)?;
    info!("{}", iformat!("### Plotting elapsed: {start.elapsed():?}"));

    info!("{}", iformat!("### Total plotting elapsed: {total_start.elapsed():?}"));

    Ok(())
}
