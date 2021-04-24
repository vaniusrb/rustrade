use crate::config::{definition::ConfigDefinition, selection::Selection};
use crate::services::provider::candles_provider::CandlesProvider;
use crate::services::provider::candles_provider_buffer::CandlesProviderBuffer;
use crate::services::provider::candles_provider_buffer_singleton::CandlesProviderBufferSingleton;
use crate::services::provider::candles_provider_selection::CandlesProviderSelection;
use crate::services::script::script_back_test::run_script;
use crate::services::technicals::top_bottom_tec::TopBottomTec;
use crate::services::trading::top_bottom_triangle::top_bottom_triangle;
use crate::utils::date_utils::datetime_to_filename;
use crate::Exchange;
use crate::Streamer;
use crate::{
    repository::candle_repository::CandleRepository,
    services::tec_plotter::plot_selection::plot_selection,
};
use chrono::Duration;
use log::info;
use sqlx::PgPool;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Application {
    pub definition: ConfigDefinition,
    pub selection: Selection,
    pub candles_provider: CandlesProviderBuffer,
}

impl Application {
    pub fn new(repository: CandleRepository, exchange: Exchange, selection: Selection) -> Self {
        let candles_provider_singleton = CandlesProviderBufferSingleton::new(repository, exchange);
        Application {
            candles_provider: CandlesProviderBuffer::new(candles_provider_singleton),
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

    pub fn run_script_test(&mut self, pool: Arc<RwLock<PgPool>>, file: &str) -> eyre::Result<()> {
        run_script(pool, self, file)?;
        Ok(())
    }

    pub fn plot_triangles(&mut self) -> eyre::Result<()> {
        let selection = self.selection.clone();
        let candles_selection = selection.candles_selection;
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
        let candles_provider_selection = CandlesProviderSelection::new(
            self.candles_provider.clone(),
            selection.candles_selection,
        );
        let candles_provider = Box::new(candles_provider_selection);
        plot_selection(selection, candles_provider, Vec::new())
    }
}

pub fn plot_triangles(
    selection: Selection,
    mut candles_provider: Box<dyn CandlesProvider>,
) -> eyre::Result<()> {
    let candles = candles_provider.candles()?;
    let topbottom_tac = TopBottomTec::new(&candles, candles.len(), 7);
    let top_bottoms = topbottom_tac.top_bottoms()?;

    let top_bottoms = top_bottoms.iter().collect::<Vec<_>>();
    let top_bottoms_ref = top_bottoms.as_slice();

    let minutes = selection.candles_selection.symbol_minutes.minutes;

    let triangles = top_bottom_triangle(top_bottoms_ref, minutes);
    triangles.iter().for_each(|triangle| {
        let mut selection_par = selection.clone();
        let open_time = triangle.open(minutes);
        let margin = Duration::minutes(minutes as i64 * 100);
        selection_par.candles_selection.start_time = open_time - margin;
        selection_par.candles_selection.end_time = open_time + margin;
        selection_par.image_name = format!("out/triangle_{}.png", datetime_to_filename(&open_time));
        info!("Plotting triangle {}", selection_par.image_name);

        plot_selection(selection_par, candles_provider.clone_provider(), Vec::new()).unwrap();
    });
    Ok(())
}
