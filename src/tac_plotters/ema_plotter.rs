use super::indicator_plotter::PlotterIndicatorContext;
use crate::{config::selection::Selection, model::candle::Candle};
use chrono::{DateTime, Utc};
use plotters::{coord::types::RangedCoordf32, prelude::*};
use plotters_bitmap::bitmap_pixel::RGBPixel;
use rust_decimal::prelude::ToPrimitive;

pub struct EmaPlotter<'a> {
    candles: &'a [&'a Candle],
}

impl<'a> EmaPlotter<'a> {
    pub fn new(candles: &'a [&'a Candle]) -> Self {
        EmaPlotter { candles }
    }
}

impl<'a> PlotterIndicatorContext for EmaPlotter<'a> {
    fn plot(
        &self,
        _selection: &Selection,
        chart_context: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>>,
    ) -> anyhow::Result<()> {
        let red = RGBColor(164, 16, 64);
        let green = RGBColor(16, 196, 64);

        chart_context.configure_mesh().x_labels(12).light_line_style(&WHITE).draw()?;

        let candle_series = self.candles.iter().map(|x| {
            CandleStick::new(
                x.close_time,
                x.open.to_f32().unwrap(),
                x.high.to_f32().unwrap(),
                x.low.to_f32().unwrap(),
                x.close.to_f32().unwrap(),
                &green,
                &red,
                2,
            )
        });
        chart_context.draw_series(candle_series)?;
        Ok(())
    }
}