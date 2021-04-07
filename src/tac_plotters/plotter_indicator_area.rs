use crate::{
    config::selection::Selection,
    service::technicals::{indicator::Indicator, technical::TechnicalIndicators},
};
use eyre::bail;
use eyre::eyre;
use log::info;
use plotters::{coord::Shift, prelude::DrawingArea, style::RGBColor};
use plotters::{
    prelude::{ChartBuilder, LabelAreaPosition, LineSeries},
    style::WHITE,
};
use plotters_bitmap::{bitmap_pixel::RGBPixel, BitMapBackend};

pub trait PlotterIndicatorArea {
    fn technical_indicators(&self) -> Box<dyn TechnicalIndicators>;

    fn plot(
        &self,
        selection: &Selection,
        lower: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    ) -> eyre::Result<()> {
        let selected_tac = selection
            .tacs
            .get(&self.technical_indicators().name())
            .ok_or_else(|| eyre!("Tac {} not selected!", self.technical_indicators().name()))?;
        let tac = self.technical_indicators();
        let mut selected_inds = Vec::new();

        if self.technical_indicators().indicators().is_empty() {
            bail!(
                "{}_tac.indicators.is_empty",
                self.technical_indicators().name()
            );
        }

        for sel_ind_name in selected_tac.indicators.iter() {
            let tac_ind = tac
                .indicators()
                .get(sel_ind_name)
                .ok_or_else(|| eyre!("Indicator {} not found!", sel_ind_name))?;
            selected_inds.push(tac_ind);
        }
        self.plot_indicators(&selected_inds, selection, lower)
    }

    fn indicator_color(&self, indicator: &Indicator) -> RGBColor;

    fn plot_indicators(
        &self,
        indicators: &[&Indicator],
        selection: &Selection,
        lower: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    ) -> eyre::Result<()> {
        let from_date = selection.candles_selection.start_time;
        let to_date = selection.candles_selection.end_time;

        let (min_macd, max_macd) = indicators
            .iter()
            .map(|i| i.min_max())
            .reduce(|p, c| (p.0.min(c.0), p.1.max(c.1)))
            .ok_or_else(|| eyre!("plot_indicators: have no min x max"))?;

        if min_macd == 0. && max_macd == 0. {
            bail!("plot_indicators: min x max values are zeros!");
        }

        let mut cart_context_lower = ChartBuilder::on(&lower)
            .set_label_area_size(LabelAreaPosition::Left, 30)
            .set_label_area_size(LabelAreaPosition::Right, 80)
            .y_label_area_size(80)
            .x_label_area_size(30)
            //   .caption(iformat!("{symbol} price"), ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(from_date..to_date, min_macd..max_macd)?;

        cart_context_lower
            .configure_mesh()
            .light_line_style(&WHITE)
            .draw()?;

        for indicator in indicators {
            info!("Plotting indicator {}", indicator.name);
            let color = self.indicator_color(indicator);
            let macd_series = LineSeries::new(
                indicator.series.iter().map(|s| (s.date_time, s.value)),
                &color,
            );
            cart_context_lower.draw_series(macd_series)?;
        }

        Ok(())
    }
}
