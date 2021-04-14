use crate::config::selection::Selection;
use crate::services::provider::candles_provider::CandlesProvider;
use crate::services::tec_plotter::candles_plotter::CandlePlotter;
use crate::services::tec_plotter::line_ind_plotter::LineIndicatorPlotter;
use crate::services::tec_plotter::macd_plotter::MacdPlotter;
use crate::services::tec_plotter::plotter::Plotter;
use crate::services::tec_plotter::plotter_indicator_context::PlotterIndicatorContext;
use crate::services::tec_plotter::rsi_plotter::RsiPlotter;
use crate::services::tec_plotter::top_bottom_plotter::TopBottomPlotter;
use crate::services::technicals::macd_tac::MacdTac;
use crate::services::technicals::rsi_tac::RsiTac;
use crate::services::technicals::technical::TechnicalIndicators;
use crate::services::technicals::top_bottom_tac::TopBottomTac;
use crate::EmaTac;
use colored::Colorize;
use ifmt::iformat;
use log::info;
use plotters::style::RGBColor;
use std::time::Instant;

pub struct PlotterSelection<'a> {
    selection: Selection,
    candles_provider: Box<dyn CandlesProvider>,
    additional_plotters: Vec<Box<dyn PlotterIndicatorContext + 'a>>,
}

impl<'a> PlotterSelection<'a> {
    pub fn from(selection: Selection, candles_provider: Box<dyn CandlesProvider>) -> Self {
        Self {
            selection,
            candles_provider,
            additional_plotters: Vec::new(),
        }
    }

    /// Push additional custom plotter
    pub fn push_plotter_ind(&mut self, plotter_indicator: Box<dyn PlotterIndicatorContext + 'a>) {
        self.additional_plotters.push(plotter_indicator);
    }

    pub fn plot(&mut self) -> eyre::Result<()> {
        let total_start = Instant::now();

        let candles_provider_clone = self.candles_provider.clone_provider();
        let candles = self.candles_provider.candles()?;

        let start_time = self.selection.candles_selection.start_time;
        let end_time = self.selection.candles_selection.end_time;

        // TODO Is possible there is any candle out of range? Is this necessary?
        // check with debug_assert!
        let candles = candles
            .into_iter()
            .filter(|c| c.open_time >= start_time && c.open_time <= end_time)
            .collect::<Vec<_>>();

        info!(
            "Plotting selection {:?} {:?} candles.len {} image {}",
            self.selection.candles_selection.start_time,
            self.selection.candles_selection.end_time,
            candles.len(),
            self.selection.image_name.green()
        );

        // TODO must obey the Selection.tacs
        // Default technicals
        let macd_tac = MacdTac::new(&candles, 34, 72, 17);
        let rsi_tac = RsiTac::new(&candles, 14);
        let ema_short_tac = EmaTac::new(&candles, 17);
        let ema_long_tac = EmaTac::new(&candles, 72);
        let mut top_bottom_tac = TopBottomTac::new(candles_provider_clone, 7);
        let top_bottoms = top_bottom_tac.top_bottoms()?;

        // Create plotter object
        let mut plotter = Plotter::new(self.selection.clone());

        // ema 17 = purple
        let short_purple = RGBColor(128, 0, 128);
        // ema 72 = orange
        let long_orange = RGBColor(255, 165, 0);
        // Upper indicators plotters
        let candle_plotter = CandlePlotter::new(&candles);
        let ema_short_plotter =
            LineIndicatorPlotter::new(ema_short_tac.main_indicator(), short_purple);
        let ema_long_plotter =
            LineIndicatorPlotter::new(ema_long_tac.main_indicator(), long_orange);
        let top_bottom_plotter = TopBottomPlotter::new(&top_bottoms);
        plotter.add_plotter_upper_ind(&candle_plotter);
        plotter.add_plotter_upper_ind(&top_bottom_plotter);
        plotter.add_plotter_upper_ind(&ema_short_plotter);
        plotter.add_plotter_upper_ind(&ema_long_plotter);

        // Custom indicators
        self.additional_plotters
            .iter()
            .for_each(|p| plotter.add_plotter_upper_ind(&**p));

        // Lower indicators plotters
        let macd_plotter = MacdPlotter::new(&macd_tac);
        plotter.add_plotter_lower_ind(&macd_plotter);

        let rsi_plotter = RsiPlotter::new(&rsi_tac);
        plotter.add_plotter_lower_ind(&rsi_plotter);

        let start = Instant::now();

        plotter.plot(&self.selection.image_name)?;
        info!("{}", iformat!("### Plotting elapsed: {start.elapsed():?}"));

        info!(
            "{}",
            iformat!("### Total plotting elapsed: {total_start.elapsed():?}")
        );

        Ok(())
    }
}

pub fn plot_selection<'a>(
    selection: Selection,
    candles_provider: Box<dyn CandlesProvider>,
    additional_plotters: Vec<Box<dyn PlotterIndicatorContext + 'a>>,
) -> eyre::Result<()> {
    let mut plotter_selection = PlotterSelection::from(selection, candles_provider);
    additional_plotters
        .into_iter()
        .for_each(|p| plotter_selection.push_plotter_ind(p));
    plotter_selection.plot()
}
