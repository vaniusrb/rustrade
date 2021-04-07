use crate::config::selection::Selection;
use chrono::{DateTime, Utc};
use plotters::prelude::{ChartContext, RangedDateTime};
use plotters::{coord::types::RangedCoordf32, prelude::Cartesian2d};
use plotters_bitmap::{bitmap_pixel::RGBPixel, BitMapBackend};

pub trait PlotterIndicatorContext {
    fn plot(
        &self,
        selection: &Selection,
        chart_context: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>,
        >,
    ) -> eyre::Result<()>;

    fn min_max(&self) -> (f64, f64);
}
