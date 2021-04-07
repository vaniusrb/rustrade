use super::indicator_plotter::IndicatorPlotter;
use crate::config::selection::Selection;
use crate::service::technicals::{indicator::Indicator, rsi_tac::RsiTac};
use eyre::{bail, eyre};
use log::info;
use plotters::prelude::*;
use plotters::{
    coord::Shift,
    prelude::{ChartBuilder, LabelAreaPosition, LineSeries},
    style::{BLACK, WHITE},
};
use plotters_bitmap::{self, bitmap_pixel::RGBPixel, BitMapBackend};

pub struct RsiPlotter<'a> {
    rsi_tac: &'a RsiTac,
}

impl<'a> RsiPlotter<'a> {
    pub fn new(rsi_tac: &'a RsiTac) -> Self {
        RsiPlotter { rsi_tac }
    }
}

impl<'a> IndicatorPlotter for RsiPlotter<'a> {
    fn plot(
        &self,
        selection: &Selection,
        upper: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
        lower: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    ) -> eyre::Result<()> {
        let selected_tac = selection
            .tacs
            .get("rsi")
            .ok_or_else(|| eyre!("Tac rsi not selected!"))?;
        let mut selected_inds = Vec::new();

        if self.rsi_tac.indicators.is_empty() {
            bail!("rsi_tac.indicators.is_empty");
        }

        for sel_ind_name in selected_tac.indicators.iter() {
            let tac_ind = self
                .rsi_tac
                .indicators
                .get(sel_ind_name)
                .ok_or_else(|| eyre!("Indicator {} not found!", sel_ind_name))?;
            selected_inds.push(tac_ind);
        }
        plot_indicators(&selected_inds, selection, upper, lower)
    }
}

fn plot_indicators(
    indicators: &[&Indicator],
    selection: &Selection,
    _upper: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    lower: &DrawingArea<BitMapBackend<RGBPixel>, Shift>,
) -> eyre::Result<()> {
    let from_date = selection.candles_selection.start_time;
    let to_date = selection.candles_selection.end_time;

    let (min_rsi, max_rsi) = indicators
        .iter()
        .map(|i| i.min_max())
        .reduce(|p, c| (p.0.min(c.0), p.1.max(c.1)))
        .ok_or_else(|| eyre!("plot_indicators: have no min x max"))?;

    if min_rsi == 0. && max_rsi == 0. {
        bail!("plot_indicators: min x max values are zeros!");
    }

    let mut cart_context_lower = ChartBuilder::on(&lower)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 80)
        .y_label_area_size(80)
        .x_label_area_size(30)
        //   .caption(iformat!("{symbol} price"), ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, min_rsi..max_rsi)?;

    cart_context_lower
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()?;

    for indicator in indicators {
        info!("Plotting indicator {}", indicator.name);
        let color = indicator_color(indicator);
        let rsi_series = LineSeries::new(
            indicator.series.iter().map(|s| (s.date_time, s.value)),
            &color,
        );
        cart_context_lower.draw_series(rsi_series)?;
    }

    Ok(())
}

fn indicator_color(indicator: &Indicator) -> RGBColor {
    match &indicator.name[..] {
        "rsi" => RGBColor(0, 0, 255),
        "signal" => RGBColor(255, 0, 0),
        _ => BLACK,
    }
}
