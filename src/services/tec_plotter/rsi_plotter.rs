use super::plotter_indicator_area::PlotterIndicatorArea;
use crate::services::technicals::technical::TecSerieIndicators;
use crate::services::technicals::{rsi_tec::RsiTec, serie_indicator::SerieIndicator};
use plotters::prelude::*;
use plotters::style::BLACK;

pub struct RsiPlotter<'a> {
    rsi_tac: &'a RsiTec,
}

impl<'a> RsiPlotter<'a> {
    pub fn new(rsi_tac: &'a RsiTec) -> Self {
        RsiPlotter { rsi_tac }
    }
}

impl<'a> PlotterIndicatorArea for RsiPlotter<'a> {
    fn indicator_color(&self, indicator: &SerieIndicator) -> RGBColor {
        match &indicator.name[..] {
            "rsi" => RGBColor(0, 0, 255),
            _ => BLACK,
        }
    }

    fn tec_serie_indicators(&self) -> &dyn TecSerieIndicators {
        self.rsi_tac as &dyn TecSerieIndicators
    }
}
