use super::plotter_indicator_area::PlotterIndicatorArea;
use crate::services::technicals::indicator::Indicator;
use crate::services::technicals::technical::TecSerieIndicators;
use crate::services::technicals::technical::TechnicalIndicators;
use crate::services::technicals::{macd_tac::MacdTac, serie_indicator::SerieIndicator};
use plotters::prelude::*;

pub struct MacdPlotter<'a> {
    macd_tac: &'a MacdTac,
}

impl<'a> MacdPlotter<'a> {
    pub fn new(macd_tac: &'a MacdTac) -> Self {
        MacdPlotter { macd_tac }
    }
}

impl<'a> PlotterIndicatorArea for MacdPlotter<'a> {
    fn indicator_color(&self, indicator: &SerieIndicator) -> RGBColor {
        match &indicator.name[..] {
            "macd" => RGBColor(0, 0, 255),
            "signal" => RGBColor(255, 0, 0),
            _ => BLACK,
        }
    }

    fn tec_serie_indicators(&self) -> &dyn TecSerieIndicators {
        self.macd_tac
    }
}
