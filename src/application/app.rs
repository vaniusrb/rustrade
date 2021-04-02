use super::{
    candles_provider::{
        CandlesProvider, CandlesProviderBuffer, CandlesProviderBufferSingleton, CandlesProviderSelection,
    },
    plot_selection::plot_selection,
    streamer::Streamer,
};
use crate::strategy::back_test_runner::run_trader_back_test;
use crate::strategy::script_back_test::run_script;
use crate::technicals::top_bottom_tac::TopBottomTac;
use crate::{
    candles_utils::datetime_to_filename,
    config::{definition::ConfigDefinition, selection::Selection},
    exchange::Exchange,
    repository::Repository,
    strategy::top_bottom_triangle::top_bottom_triangle,
};
use chrono::Duration;
use log::info;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Application {
    pub definition: ConfigDefinition,
    pub selection: Selection,
    pub candles_provider: CandlesProviderBuffer,
}

impl Application {
    pub fn new(repository: Repository, exchange: Exchange, selection: Selection) -> Self {
        let candles_provider_singleton = CandlesProviderBufferSingleton::new(repository, exchange);
        Application {
            candles_provider: CandlesProviderBuffer::new(Arc::new(RwLock::new(candles_provider_singleton))),
            selection,
            definition: ConfigDefinition::new(),
        }
    }

    pub fn definition(&self) -> &ConfigDefinition {
        &self.definition
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = selection;
    }

    pub fn run_back_test(&mut self) -> eyre::Result<()> {
        run_trader_back_test(self)?;
        Ok(())
    }

    pub fn run_script_test(&mut self, file: &str) -> eyre::Result<()> {
        run_script(self, file)?;
        Ok(())
    }

    pub fn plot_triangles(&mut self) -> eyre::Result<()> {
        let selection = self.selection.clone();
        let candles_selection = selection.candles_selection.clone();
        let candles_provider_selection =
            CandlesProviderSelection::new(self.candles_provider.clone(), candles_selection);
        let candles_provider = Box::new(candles_provider_selection);
        plot_triangles(selection, candles_provider)
    }

    pub fn run_stream(&mut self) -> eyre::Result<()> {
        let mut streamer = Streamer::new(self);
        streamer.run()
    }

    pub fn plot_selection(&mut self) -> eyre::Result<()> {
        let selection = self.selection.clone();
        let candles_provider_selection =
            CandlesProviderSelection::new(self.candles_provider.clone(), selection.candles_selection.clone());
        let candles_provider = Box::new(candles_provider_selection);
        plot_selection(selection, candles_provider, Vec::new())
    }
}

pub fn plot_triangles(selection: Selection, candles_provider: Box<dyn CandlesProvider>) -> eyre::Result<()> {
    let mut topbottom_tac = TopBottomTac::new(candles_provider.clone_provider(), 7);
    let top_bottoms = topbottom_tac.top_bottoms()?;

    let top_bottoms = top_bottoms.iter().collect::<Vec<_>>();
    let top_bottoms_ref = top_bottoms.as_slice();

    let minutes = selection.candles_selection.symbol_minutes.minutes;

    let triangles = top_bottom_triangle(top_bottoms_ref, &minutes);
    triangles.iter().for_each(|triangle| {
        let mut selection_par = selection.clone();
        let open_time = triangle.open(&minutes);
        let margin = Duration::minutes(minutes as i64 * 100);
        selection_par.candles_selection.start_time = open_time - margin;
        selection_par.candles_selection.end_time = open_time + margin;
        selection_par.image_name = format!("out/triangle_{}.png", datetime_to_filename(&open_time));
        info!("Plotting triangle {}", selection_par.image_name);

        plot_selection(selection_par, candles_provider.clone_provider(), Vec::new()).unwrap();
    });
    Ok(())
}
